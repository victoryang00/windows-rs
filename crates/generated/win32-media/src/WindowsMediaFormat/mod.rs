#[repr(C)]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows_core::HRESULT,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AM_WMT_EVENT_DATA {}
impl ::core::clone::Clone for AM_WMT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AM_WMT_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AM_WMT_EVENT_DATA").field("hrStatus", &self.hrStatus).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows_core::Abi for AM_WMT_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AM_WMT_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for AM_WMT_EVENT_DATA {}
impl ::core::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_ClientNetManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd12a3ce_9c42_11d2_beed_0060082f2054);
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[repr(C)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl ::core::marker::Copy for DRM_COPY_OPL {}
impl ::core::clone::Clone for DRM_COPY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_COPY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_COPY_OPL").field("wMinimumCopyLevel", &self.wMinimumCopyLevel).field("oplIdIncludes", &self.oplIdIncludes).field("oplIdExcludes", &self.oplIdExcludes).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_COPY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_COPY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_COPY_OPL {}
impl ::core::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl ::core::marker::Copy for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::clone::Clone for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS").field("wCompressedDigitalVideo", &self.wCompressedDigitalVideo).field("wUncompressedDigitalVideo", &self.wUncompressedDigitalVideo).field("wAnalogVideo", &self.wAnalogVideo).field("wCompressedDigitalAudio", &self.wCompressedDigitalAudio).field("wUncompressedDigitalAudio", &self.wUncompressedDigitalAudio).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for DRM_OPL_OUTPUT_IDS {}
impl ::core::clone::Clone for DRM_OPL_OUTPUT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OPL_OUTPUT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OPL_OUTPUT_IDS").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_OPL_OUTPUT_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OPL_OUTPUT_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
impl ::core::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DRM_OPL_TYPES: u32 = 1u32;
#[repr(C)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows_core::GUID,
    pub bConfigData: u8,
}
impl ::core::marker::Copy for DRM_OUTPUT_PROTECTION {}
impl ::core::clone::Clone for DRM_OUTPUT_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OUTPUT_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OUTPUT_PROTECTION").field("guidId", &self.guidId).field("bConfigData", &self.bConfigData).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_OUTPUT_PROTECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OUTPUT_PROTECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OUTPUT_PROTECTION {}
impl ::core::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl ::core::marker::Copy for DRM_PLAY_OPL {}
impl ::core::clone::Clone for DRM_PLAY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_PLAY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_PLAY_OPL").field("minOPL", &self.minOPL).field("oplIdReserved", &self.oplIdReserved).field("vopi", &self.vopi).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_PLAY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_PLAY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_PLAY_OPL {}
impl ::core::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DRM_VAL16 {}
impl ::core::clone::Clone for DRM_VAL16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VAL16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VAL16").field("val", &self.val).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_VAL16 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VAL16>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VAL16 {}
impl ::core::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl ::core::marker::Copy for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::clone::Clone for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VIDEO_OUTPUT_PROTECTION_IDS").field("cEntries", &self.cEntries).field("rgVop", &self.rgVop).finish()
    }
}
unsafe impl ::windows_core::Abi for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VIDEO_OUTPUT_PROTECTION_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct INSNetSourceCreator(::windows_core::IUnknown);
impl INSNetSourceCreator {
    pub unsafe fn Initialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNetSource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param4: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pszstreamname: Param0, pmonitor: Param1, pdata: *const u8, pusercontext: Param3, pcallback: Param4, qwcontext: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateNetSource)(::windows_core::Interface::as_raw(self), pszstreamname.into_param().abi(), pmonitor.into_param().abi(), ::core::mem::transmute(pdata), pusercontext.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(qwcontext)).ok()
    }
    pub unsafe fn GetNetSourceProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszstreamname: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetSourceProperties)(::windows_core::Interface::as_raw(self), pszstreamname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetSourceSharedNamespace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNetSourceAdminInterface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszstreamname: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetSourceAdminInterface)(::windows_core::Interface::as_raw(self), pszstreamname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNumProtocolsSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<INSNetSourceCreator> for ::windows_core::IUnknown {
    fn from(value: INSNetSourceCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSNetSourceCreator> for ::windows_core::IUnknown {
    fn from(value: &INSNetSourceCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INSNetSourceCreator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INSNetSourceCreator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSNetSourceCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSNetSourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSNetSourceCreator {}
impl ::core::fmt::Debug for INSNetSourceCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSNetSourceCreator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INSNetSourceCreator {
    type Vtable = INSNetSourceCreator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c0e4080_9081_11d2_beec_0060082f2054);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSNetSourceCreator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateNetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows_core::HRESULT,
    pub GetNetSourceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNetSourceSharedNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNetSourceAdminInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows_core::PCWSTR, pval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNetSourceAdminInterface: usize,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows_core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u16) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INSSBuffer(::windows_core::IUnknown);
impl INSSBuffer {
    pub unsafe fn GetLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBufferAndLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
}
impl ::core::convert::From<INSSBuffer> for ::windows_core::IUnknown {
    fn from(value: INSSBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer> for ::windows_core::IUnknown {
    fn from(value: &INSSBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INSSBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INSSBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer {}
impl ::core::fmt::Debug for INSSBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INSSBuffer {
    type Vtable = INSSBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INSSBuffer2(::windows_core::IUnknown);
impl INSSBuffer2 {
    pub unsafe fn GetLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBufferAndLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
}
impl ::core::convert::From<INSSBuffer2> for ::windows_core::IUnknown {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for ::windows_core::IUnknown {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INSSBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INSSBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer2> for INSSBuffer {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for INSSBuffer {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for INSSBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer2 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer2 {}
impl ::core::fmt::Debug for INSSBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INSSBuffer2 {
    type Vtable = INSSBuffer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f528693_1035_43fe_b428_757561ad3a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_Vtbl {
    pub base__: INSSBuffer_Vtbl,
    pub GetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows_core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INSSBuffer3(::windows_core::IUnknown);
impl INSSBuffer3 {
    pub unsafe fn GetLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferAndLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    pub unsafe fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer3> for ::windows_core::IUnknown {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for ::windows_core::IUnknown {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer2 {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer2 {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer2> for INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer2> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer3 {}
impl ::core::fmt::Debug for INSSBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INSSBuffer3 {
    type Vtable = INSSBuffer3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_Vtbl {
    pub base__: INSSBuffer2_Vtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows_core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INSSBuffer4(::windows_core::IUnknown);
impl INSSBuffer4 {
    pub unsafe fn GetLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetMaxLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetBufferAndLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows_core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSampleProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    pub unsafe fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwbufferpropertyindex), ::core::mem::transmute(pguidbufferproperty), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer4> for ::windows_core::IUnknown {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for ::windows_core::IUnknown {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer2 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer2 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer2> for INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer2> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer3 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer3 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer3> for INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, INSSBuffer3> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows_core::Param<'a, INSSBuffer3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer4 {}
impl ::core::fmt::Debug for INSSBuffer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer4").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INSSBuffer4 {
    type Vtable = INSSBuffer4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_Vtbl {
    pub base__: INSSBuffer3_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows_core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMAddressAccess(::windows_core::IUnknown);
impl IWMAddressAccess {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessEntryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_ADDRESS_ACCESSENTRY>::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess> for ::windows_core::IUnknown {
    fn from(value: IWMAddressAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess> for ::windows_core::IUnknown {
    fn from(value: &IWMAddressAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMAddressAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMAddressAccess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess {}
impl ::core::fmt::Debug for IWMAddressAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMAddressAccess {
    type Vtable = IWMAddressAccess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAccessEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows_core::HRESULT,
    pub GetAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows_core::HRESULT,
    pub AddAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_core::HRESULT,
    pub RemoveAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMAddressAccess2(::windows_core::IUnknown);
impl IWMAddressAccess2 {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAccessEntryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_ADDRESS_ACCESSENTRY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveAccessEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::win32_foundation::BSTR, pbstrmask: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAccessEntryEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(pbstraddress), ::core::mem::transmute(pbstrmask)).ok()
    }
    pub unsafe fn AddAccessEntryEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, aetype: WM_AETYPE, bstraddress: Param1, bstrmask: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAccessEntryEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(aetype), bstraddress.into_param().abi(), bstrmask.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess2> for ::windows_core::IUnknown {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for ::windows_core::IUnknown {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMAddressAccess2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMAddressAccess> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMAddressAccess> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMAddressAccess> for &'a IWMAddressAccess2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMAddressAccess> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess2 {}
impl ::core::fmt::Debug for IWMAddressAccess2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMAddressAccess2 {
    type Vtable = IWMAddressAccess2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2_Vtbl {
    pub base__: IWMAddressAccess_Vtbl,
    pub GetAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::win32_foundation::BSTR, pbstrmask: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub AddAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMAuthorizer(::windows_core::IUnknown);
impl IWMAuthorizer {
    pub unsafe fn GetCertCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCertCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetCert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).GetSharedData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<IWMAuthorizer> for ::windows_core::IUnknown {
    fn from(value: IWMAuthorizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAuthorizer> for ::windows_core::IUnknown {
    fn from(value: &IWMAuthorizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMAuthorizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMAuthorizer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAuthorizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAuthorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAuthorizer {}
impl ::core::fmt::Debug for IWMAuthorizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAuthorizer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMAuthorizer {
    type Vtable = IWMAuthorizer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9b67d36_a9ad_4eb4_baef_db284ef5504c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAuthorizer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCertCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows_core::HRESULT,
    pub GetCert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows_core::HRESULT,
    pub GetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMBackupRestoreProps(::windows_core::IUnknown);
impl IWMBackupRestoreProps {
    pub unsafe fn GetPropCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn GetPropByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropByName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetProp<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProp)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn RemoveProp<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pcwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveProp)(::windows_core::Interface::as_raw(self), pcwszname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllProps(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllProps)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMBackupRestoreProps> for ::windows_core::IUnknown {
    fn from(value: IWMBackupRestoreProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBackupRestoreProps> for ::windows_core::IUnknown {
    fn from(value: &IWMBackupRestoreProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMBackupRestoreProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMBackupRestoreProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBackupRestoreProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBackupRestoreProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBackupRestoreProps {}
impl ::core::fmt::Debug for IWMBackupRestoreProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBackupRestoreProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMBackupRestoreProps {
    type Vtable = IWMBackupRestoreProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestoreProps_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows_core::HRESULT,
    pub GetPropByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub GetPropByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub SetProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
    pub RemoveProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMBandwidthSharing(::windows_core::IUnknown);
impl IWMBandwidthSharing {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetStreams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype)).ok()
    }
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwbitrate), ::core::mem::transmute(pmsbufferwindow)).ok()
    }
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwbitrate), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for ::windows_core::IUnknown {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for ::windows_core::IUnknown {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMBandwidthSharing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for IWMStreamList {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for IWMStreamList {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for &'a IWMBandwidthSharing {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBandwidthSharing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBandwidthSharing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBandwidthSharing {}
impl ::core::fmt::Debug for IWMBandwidthSharing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBandwidthSharing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMBandwidthSharing {
    type Vtable = IWMBandwidthSharing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharing_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows_core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMClientConnections(::windows_core::IUnknown);
impl IWMClientConnections {
    pub unsafe fn GetClientCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetClientCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows_core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_CLIENT_PROPERTIES>::zeroed();
        (::windows_core::Interface::vtable(self).GetClientProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
}
impl ::core::convert::From<IWMClientConnections> for ::windows_core::IUnknown {
    fn from(value: IWMClientConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections> for ::windows_core::IUnknown {
    fn from(value: &IWMClientConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMClientConnections {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMClientConnections {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections {}
impl ::core::fmt::Debug for IWMClientConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMClientConnections {
    type Vtable = IWMClientConnections_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows_core::HRESULT,
    pub GetClientProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMClientConnections2(::windows_core::IUnknown);
impl IWMClientConnections2 {
    pub unsafe fn GetClientCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClientCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows_core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_CLIENT_PROPERTIES>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClientProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: ::windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows_core::PWSTR, pcchdnsname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(pwsznetworkaddress), ::core::mem::transmute(pcchnetworkaddress), ::core::mem::transmute(pwszport), ::core::mem::transmute(pcchport), ::core::mem::transmute(pwszdnsname), ::core::mem::transmute(pcchdnsname)).ok()
    }
}
impl ::core::convert::From<IWMClientConnections2> for ::windows_core::IUnknown {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for ::windows_core::IUnknown {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMClientConnections2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMClientConnections2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMClientConnections2> for IWMClientConnections {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for IWMClientConnections {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMClientConnections> for IWMClientConnections2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMClientConnections> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMClientConnections> for &'a IWMClientConnections2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMClientConnections> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections2 {}
impl ::core::fmt::Debug for IWMClientConnections2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMClientConnections2 {
    type Vtable = IWMClientConnections2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2_Vtbl {
    pub base__: IWMClientConnections_Vtbl,
    pub GetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: ::windows_core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows_core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows_core::PWSTR, pcchdnsname: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMCodecInfo(::windows_core::IUnknown);
impl IWMCodecInfo {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodecInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodecFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodecFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
}
impl ::core::convert::From<IWMCodecInfo> for ::windows_core::IUnknown {
    fn from(value: IWMCodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo> for ::windows_core::IUnknown {
    fn from(value: &IWMCodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMCodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMCodecInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo {}
impl ::core::fmt::Debug for IWMCodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMCodecInfo {
    type Vtable = IWMCodecInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, pccodecs: *mut u32) -> ::windows_core::HRESULT,
    pub GetCodecFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows_core::HRESULT,
    pub GetCodecFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMCodecInfo2(::windows_core::IUnknown);
impl IWMCodecInfo2 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodecInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodecFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodecFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, wszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows_core::PWSTR, pcchdesc: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecFormatDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo2> for ::windows_core::IUnknown {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for ::windows_core::IUnknown {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMCodecInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo> for &'a IWMCodecInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo2 {}
impl ::core::fmt::Debug for IWMCodecInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMCodecInfo2 {
    type Vtable = IWMCodecInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2_Vtbl {
    pub base__: IWMCodecInfo_Vtbl,
    pub GetCodecName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, wszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::HRESULT,
    pub GetCodecFormatDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows_core::RawPtr, wszdesc: ::windows_core::PWSTR, pcchdesc: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMCodecInfo3(::windows_core::IUnknown);
impl IWMCodecInfo3 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetCodecInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetCodecFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetCodecFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, wszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCodecName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows_core::PWSTR, pcchdesc: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCodecFormatDesc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
    pub unsafe fn GetCodecFormatProp<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: Param3, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecFormatProp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    pub unsafe fn GetCodecProp<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecProp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    pub unsafe fn SetCodecEnumerationSetting<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: Param2, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCodecEnumerationSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetCodecEnumerationSetting<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecEnumerationSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo3> for ::windows_core::IUnknown {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for ::windows_core::IUnknown {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo2> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMCodecInfo2> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMCodecInfo2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo3 {}
impl ::core::fmt::Debug for IWMCodecInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMCodecInfo3 {
    type Vtable = IWMCodecInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3_Vtbl {
    pub base__: IWMCodecInfo2_Vtbl,
    pub GetCodecFormatProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub GetCodecProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::HRESULT,
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID, dwcodecindex: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMCredentialCallback(::windows_core::IUnknown);
impl IWMCredentialCallback {
    pub unsafe fn AcquireCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszrealm: Param0, pwszsite: Param1, pwszuser: &mut [u16], pwszpassword: &mut [u16], hrstatus: ::windows_core::HRESULT, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireCredentials)(::windows_core::Interface::as_raw(self), pwszrealm.into_param().abi(), pwszsite.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwszuser)), pwszuser.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pwszpassword)), pwszpassword.len() as _, ::core::mem::transmute(hrstatus), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IWMCredentialCallback> for ::windows_core::IUnknown {
    fn from(value: IWMCredentialCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCredentialCallback> for ::windows_core::IUnknown {
    fn from(value: &IWMCredentialCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMCredentialCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMCredentialCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCredentialCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCredentialCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCredentialCallback {}
impl ::core::fmt::Debug for IWMCredentialCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCredentialCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMCredentialCallback {
    type Vtable = IWMCredentialCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AcquireCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszrealm: ::windows_core::PCWSTR, pwszsite: ::windows_core::PCWSTR, pwszuser: ::windows_core::PWSTR, cchuser: u32, pwszpassword: ::windows_core::PWSTR, cchpassword: u32, hrstatus: ::windows_core::HRESULT, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMEditor(::windows_core::IUnknown);
impl IWMDRMEditor {
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMEditor> for ::windows_core::IUnknown {
    fn from(value: IWMDRMEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMEditor> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMEditor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMEditor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMEditor {}
impl ::core::fmt::Debug for IWMDRMEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMEditor {
    type Vtable = IWMDRMEditor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditor_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMMessageParser(::windows_core::IUnknown);
impl IWMDRMMessageParser {
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: &[u8], ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ParseRegistrationReqMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbregistrationreqmsg)), pbregistrationreqmsg.len() as _, ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber)).ok()
    }
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: &[u8], ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ParseLicenseRequestMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pblicenserequestmsg)), pblicenserequestmsg.len() as _, ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber), ::core::mem::transmute(pbstraction)).ok()
    }
}
impl ::core::convert::From<IWMDRMMessageParser> for ::windows_core::IUnknown {
    fn from(value: IWMDRMMessageParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMMessageParser> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMMessageParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMMessageParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMMessageParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMMessageParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMMessageParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMMessageParser {}
impl ::core::fmt::Debug for IWMDRMMessageParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMMessageParser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMMessageParser {
    type Vtable = IWMDRMMessageParser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParser_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows_core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows_core::HRESULT,
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows_core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMReader(::windows_core::IUnknown);
impl IWMDRMReader {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcquireLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Individualize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelIndividualization)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelMonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader> for ::windows_core::IUnknown {
    fn from(value: IWMDRMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader {}
impl ::core::fmt::Debug for IWMDRMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMReader {
    type Vtable = IWMDRMReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AcquireLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Individualize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub CancelIndividualization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows_core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMReader2(::windows_core::IUnknown);
impl IWMDRMReader2 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AcquireLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Individualize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelIndividualization)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelMonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEvaluateOutputLevelLicenses)(::windows_core::Interface::as_raw(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPlayOutputLevels)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCopyOutputLevels)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TryNextLicense)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader2> for ::windows_core::IUnknown {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader2> for IWMDRMReader {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for IWMDRMReader {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader> for IWMDRMReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader> for &'a IWMDRMReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader2 {}
impl ::core::fmt::Debug for IWMDRMReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMReader2 {
    type Vtable = IWMDRMReader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2_Vtbl {
    pub base__: IWMDRMReader_Vtbl,
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fevaluate: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetPlayOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::HRESULT,
    pub GetCopyOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::HRESULT,
    pub TryNextLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMReader3(::windows_core::IUnknown);
impl IWMDRMReader3 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AcquireLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CancelLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Individualize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CancelIndividualization)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.MonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CancelMonitorLicenseAcquisition)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDRMProperty)(::windows_core::Interface::as_raw(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEvaluateOutputLevelLicenses)(::windows_core::Interface::as_raw(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPlayOutputLevels)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCopyOutputLevels)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TryNextLicense)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut ::windows_core::GUID, pcguids: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInclusionList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppguids), ::core::mem::transmute(pcguids)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader3> for ::windows_core::IUnknown {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader> for IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader2> for IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMReader2> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMReader2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader3 {}
impl ::core::fmt::Debug for IWMDRMReader3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMReader3 {
    type Vtable = IWMDRMReader3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3_Vtbl {
    pub base__: IWMDRMReader2_Vtbl,
    pub GetInclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows_core::GUID, pcguids: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMTranscryptionManager(::windows_core::IUnknown);
impl IWMDRMTranscryptionManager {
    pub unsafe fn CreateTranscryptor(&self) -> ::windows_core::Result<IWMDRMTranscryptor> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTranscryptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMDRMTranscryptor>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptionManager> for ::windows_core::IUnknown {
    fn from(value: IWMDRMTranscryptionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptionManager> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMTranscryptionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptionManager {}
impl ::core::fmt::Debug for IWMDRMTranscryptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMTranscryptionManager {
    type Vtable = IWMDRMTranscryptionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateTranscryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMTranscryptor(::windows_core::IUnknown);
impl IWMDRMTranscryptor {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Seek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hnstime)).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMTranscryptor> for ::windows_core::IUnknown {
    fn from(value: IWMDRMTranscryptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMTranscryptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMTranscryptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMTranscryptor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor {}
impl ::core::fmt::Debug for IWMDRMTranscryptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMTranscryptor {
    type Vtable = IWMDRMTranscryptor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMTranscryptor2(::windows_core::IUnknown);
impl IWMDRMTranscryptor2 {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hnstime)).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SeekEx<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SeekEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration), ::core::mem::transmute(flrate), fincludefileheader.into_param().abi()).ok()
    }
    pub unsafe fn ZeroAdjustTimestamps<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenable: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ZeroAdjustTimestamps)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetSeekStartTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetSeekStartTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetDuration(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for ::windows_core::IUnknown {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMTranscryptor> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMTranscryptor> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMTranscryptor> for &'a IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMTranscryptor> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor2 {}
impl ::core::fmt::Debug for IWMDRMTranscryptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMTranscryptor2 {
    type Vtable = IWMDRMTranscryptor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2_Vtbl {
    pub base__: IWMDRMTranscryptor_Vtbl,
    pub SeekEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetSeekStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMWriter(::windows_core::IUnknown);
impl IWMDRMWriter {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateKeySeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateKeyID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateSigningKeyPair)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDRMAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter> for ::windows_core::IUnknown {
    fn from(value: IWMDRMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter {}
impl ::core::fmt::Debug for IWMDRMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMWriter {
    type Vtable = IWMDRMWriter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GenerateKeySeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::HRESULT,
    pub GenerateKeyID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::HRESULT,
    pub GenerateSigningKeyPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::HRESULT,
    pub SetDRMAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMWriter2(::windows_core::IUnknown);
impl IWMDRMWriter2 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GenerateKeySeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GenerateKeyID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GenerateSigningKeyPair)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDRMAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWMDRMNetEncryption)(::windows_core::Interface::as_raw(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter2> for ::windows_core::IUnknown {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMWriter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter> for &'a IWMDRMWriter2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter2 {}
impl ::core::fmt::Debug for IWMDRMWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMWriter2 {
    type Vtable = IWMDRMWriter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2_Vtbl {
    pub base__: IWMDRMWriter_Vtbl,
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsamplesencrypted: ::win32_foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDRMWriter3(::windows_core::IUnknown);
impl IWMDRMWriter3 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GenerateKeySeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows_core::PWSTR, pcwchlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GenerateKeyID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows_core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows_core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GenerateSigningKeyPair)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDRMAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWMDRMNetEncryption)(::windows_core::Interface::as_raw(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtectStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimportinitstruct)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter3> for ::windows_core::IUnknown {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for ::windows_core::IUnknown {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter2> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMDRMWriter2> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMDRMWriter2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter3 {}
impl ::core::fmt::Debug for IWMDRMWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDRMWriter3 {
    type Vtable = IWMDRMWriter3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3_Vtbl {
    pub base__: IWMDRMWriter2_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMDeviceRegistration(::windows_core::IUnknown);
impl IWMDeviceRegistration {
    pub unsafe fn RegisterDevice<'a, Param3: ::windows_core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn UnregisterDevice<'a, Param3: ::windows_core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi()).ok()
    }
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetRegistrationStats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFirstRegisteredDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn GetNextRegisteredDevice(&self) -> ::windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNextRegisteredDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn GetRegisteredDeviceByID<'a, Param3: ::windows_core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows_core::Result<IWMRegisteredDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRegisteredDeviceByID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMRegisteredDevice>(result__)
    }
}
impl ::core::convert::From<IWMDeviceRegistration> for ::windows_core::IUnknown {
    fn from(value: IWMDeviceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceRegistration> for ::windows_core::IUnknown {
    fn from(value: &IWMDeviceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMDeviceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMDeviceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDeviceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceRegistration {}
impl ::core::fmt::Debug for IWMDeviceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMDeviceRegistration {
    type Vtable = IWMDeviceRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistration_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows_core::HRESULT,
    pub GetRegistrationStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows_core::HRESULT,
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMGetSecureChannel(::windows_core::IUnknown);
impl IWMGetSecureChannel {
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> ::windows_core::Result<IWMSecureChannel> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPeerSecureChannelInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMSecureChannel>(result__)
    }
}
impl ::core::convert::From<IWMGetSecureChannel> for ::windows_core::IUnknown {
    fn from(value: IWMGetSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMGetSecureChannel> for ::windows_core::IUnknown {
    fn from(value: &IWMGetSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMGetSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMGetSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMGetSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMGetSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMGetSecureChannel {}
impl ::core::fmt::Debug for IWMGetSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMGetSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMGetSecureChannel {
    type Vtable = IWMGetSecureChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94bc0598_c3d2_11d3_bedf_00c04f612986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMGetSecureChannel_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPeerSecureChannelInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMHeaderInfo(::windows_core::IUnknown);
impl IWMHeaderInfo {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetMarkerCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    pub unsafe fn AddMarker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMarker)(::windows_core::Interface::as_raw(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetScriptCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    pub unsafe fn AddScript<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddScript)(::windows_core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo> for ::windows_core::IUnknown {
    fn from(value: IWMHeaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo> for ::windows_core::IUnknown {
    fn from(value: &IWMHeaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMHeaderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMHeaderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo {}
impl ::core::fmt::Debug for IWMHeaderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMHeaderInfo {
    type Vtable = IWMHeaderInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
    pub GetMarkerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows_core::HRESULT,
    pub GetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::HRESULT,
    pub AddMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: ::windows_core::PCWSTR, cnsmarkertime: u64) -> ::windows_core::HRESULT,
    pub RemoveMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows_core::HRESULT,
    pub GetScriptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows_core::HRESULT,
    pub GetScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::HRESULT,
    pub AddScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: ::windows_core::PCWSTR, pwszcommand: ::windows_core::PCWSTR, cnsscripttime: u64) -> ::windows_core::HRESULT,
    pub RemoveScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMHeaderInfo2(::windows_core::IUnknown);
impl IWMHeaderInfo2 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAttributeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributeByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributeByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMarkerCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    pub unsafe fn AddMarker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMarker)(::windows_core::Interface::as_raw(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetScriptCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    pub unsafe fn AddScript<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddScript)(::windows_core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCodecInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for ::windows_core::IUnknown {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for ::windows_core::IUnknown {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMHeaderInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo> for &'a IWMHeaderInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo2 {}
impl ::core::fmt::Debug for IWMHeaderInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMHeaderInfo2 {
    type Vtable = IWMHeaderInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2_Vtbl {
    pub base__: IWMHeaderInfo_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows_core::HRESULT,
    pub GetCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMHeaderInfo3(::windows_core::IUnknown);
impl IWMHeaderInfo3 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows_core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributeByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributeByName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMarkerCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows_core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    pub unsafe fn AddMarker<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddMarker)(::windows_core::Interface::as_raw(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetScriptCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows_core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows_core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    pub unsafe fn AddScript<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddScript)(::windows_core::Interface::as_raw(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodecInfoCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCodecInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributeCountEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetAttributeIndices<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pwszname: Param1, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeIndices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pwszname.into_param().abi(), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pwindices), ::core::mem::transmute(pwcount)).ok()
    }
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: ::windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeByIndexEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwdatalength)).ok()
    }
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn AddAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(pwindex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn AddCodecInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0, pwszdescription: Param1, codectype: WMT_CODEC_INFO_TYPE, pbcodecinfo: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddCodecInfo)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi(), pwszdescription.into_param().abi(), ::core::mem::transmute(codectype), pbcodecinfo.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcodecinfo))).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for ::windows_core::IUnknown {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for ::windows_core::IUnknown {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo2> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMHeaderInfo2> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMHeaderInfo2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo3 {}
impl ::core::fmt::Debug for IWMHeaderInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMHeaderInfo3 {
    type Vtable = IWMHeaderInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3_Vtbl {
    pub base__: IWMHeaderInfo2_Vtbl,
    pub GetAttributeCountEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows_core::HRESULT,
    pub GetAttributeIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: ::windows_core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows_core::HRESULT,
    pub GetAttributeByIndexEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: ::windows_core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows_core::HRESULT,
    pub ModifyAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::HRESULT,
    pub AddAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows_core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows_core::HRESULT,
    pub DeleteAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows_core::HRESULT,
    pub AddCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszdescription: ::windows_core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMIStreamProps(::windows_core::IUnknown);
impl IWMIStreamProps {
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMIStreamProps> for ::windows_core::IUnknown {
    fn from(value: IWMIStreamProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIStreamProps> for ::windows_core::IUnknown {
    fn from(value: &IWMIStreamProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMIStreamProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMIStreamProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIStreamProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIStreamProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIStreamProps {}
impl ::core::fmt::Debug for IWMIStreamProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIStreamProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMIStreamProps {
    type Vtable = IWMIStreamProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamProps_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMImageInfo(::windows_core::IUnknown);
impl IWMImageInfo {
    pub unsafe fn GetImageCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchmimetype), ::core::mem::transmute(pwszmimetype), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pimagetype), ::core::mem::transmute(pcbimagedata), ::core::mem::transmute(pbimagedata)).ok()
    }
}
impl ::core::convert::From<IWMImageInfo> for ::windows_core::IUnknown {
    fn from(value: IWMImageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMImageInfo> for ::windows_core::IUnknown {
    fn from(value: &IWMImageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMImageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMImageInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMImageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMImageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMImageInfo {}
impl ::core::fmt::Debug for IWMImageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMImageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMImageInfo {
    type Vtable = IWMImageInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows_core::HRESULT,
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows_core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows_core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMIndexer(::windows_core::IUnknown);
impl IWMIndexer {
    pub unsafe fn StartIndexing<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartIndexing)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMIndexer> for ::windows_core::IUnknown {
    fn from(value: IWMIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer> for ::windows_core::IUnknown {
    fn from(value: &IWMIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer {}
impl ::core::fmt::Debug for IWMIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMIndexer {
    type Vtable = IWMIndexer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub StartIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMIndexer2(::windows_core::IUnknown);
impl IWMIndexer2 {
    pub unsafe fn StartIndexing<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartIndexing)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Configure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(nindexertype), ::core::mem::transmute(pvinterval), ::core::mem::transmute(pvindextype)).ok()
    }
}
impl ::core::convert::From<IWMIndexer2> for ::windows_core::IUnknown {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for ::windows_core::IUnknown {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMIndexer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMIndexer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMIndexer2> for IWMIndexer {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for IWMIndexer {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMIndexer> for IWMIndexer2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMIndexer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMIndexer> for &'a IWMIndexer2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMIndexer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer2 {}
impl ::core::fmt::Debug for IWMIndexer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMIndexer2 {
    type Vtable = IWMIndexer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2_Vtbl {
    pub base__: IWMIndexer_Vtbl,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMInputMediaProps(::windows_core::IUnknown);
impl IWMInputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConnectionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn GetGroupName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGroupName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMInputMediaProps> for ::windows_core::IUnknown {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for ::windows_core::IUnknown {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMInputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMInputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMInputMediaProps> for IWMMediaProps {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for IWMMediaProps {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for IWMInputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for &'a IWMInputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMInputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMInputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInputMediaProps {}
impl ::core::fmt::Debug for IWMInputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMInputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMInputMediaProps {
    type Vtable = IWMInputMediaProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT,
    pub GetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMLanguageList(::windows_core::IUnknown);
impl IWMLanguageList {
    pub unsafe fn GetLanguageCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLanguageDetails)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn AddLanguageByRFC1766String<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).AddLanguageByRFC1766String)(::windows_core::Interface::as_raw(self), pwszlanguagestring.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
}
impl ::core::convert::From<IWMLanguageList> for ::windows_core::IUnknown {
    fn from(value: IWMLanguageList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLanguageList> for ::windows_core::IUnknown {
    fn from(value: &IWMLanguageList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMLanguageList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMLanguageList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLanguageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLanguageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLanguageList {}
impl ::core::fmt::Debug for IWMLanguageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLanguageList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMLanguageList {
    type Vtable = IWMLanguageList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows_core::HRESULT,
    pub GetLanguageDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT,
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PCWSTR, pwindex: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMLicenseBackup(::windows_core::IUnknown);
impl IWMLicenseBackup {
    pub unsafe fn BackupLicenses<'a, Param1: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackupLicenses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseBackup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelLicenseBackup)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseBackup> for ::windows_core::IUnknown {
    fn from(value: IWMLicenseBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseBackup> for ::windows_core::IUnknown {
    fn from(value: &IWMLicenseBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMLicenseBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMLicenseBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseBackup {}
impl ::core::fmt::Debug for IWMLicenseBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseBackup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMLicenseBackup {
    type Vtable = IWMLicenseBackup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackup_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub BackupLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelLicenseBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMLicenseRestore(::windows_core::IUnknown);
impl IWMLicenseRestore {
    pub unsafe fn RestoreLicenses<'a, Param1: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreLicenses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseRestore(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelLicenseRestore)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRestore> for ::windows_core::IUnknown {
    fn from(value: IWMLicenseRestore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRestore> for ::windows_core::IUnknown {
    fn from(value: &IWMLicenseRestore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMLicenseRestore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMLicenseRestore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRestore {}
impl ::core::fmt::Debug for IWMLicenseRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRestore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMLicenseRestore {
    type Vtable = IWMLicenseRestore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestore_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RestoreLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CancelLicenseRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMLicenseRevocationAgent(::windows_core::IUnknown);
impl IWMLicenseRevocationAgent {
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLRBChallenge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmachineid), ::core::mem::transmute(dwmachineidlength), ::core::mem::transmute(pchallenge), ::core::mem::transmute(dwchallengelength), ::core::mem::transmute(pchallengeoutput), ::core::mem::transmute(pdwchallengeoutputlength)).ok()
    }
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProcessLRB)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psignedlrb), ::core::mem::transmute(dwsignedlrblength), ::core::mem::transmute(psignedack), ::core::mem::transmute(pdwsignedacklength)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRevocationAgent> for ::windows_core::IUnknown {
    fn from(value: IWMLicenseRevocationAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRevocationAgent> for ::windows_core::IUnknown {
    fn from(value: &IWMLicenseRevocationAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRevocationAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRevocationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRevocationAgent {}
impl ::core::fmt::Debug for IWMLicenseRevocationAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRevocationAgent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMLicenseRevocationAgent {
    type Vtable = IWMLicenseRevocationAgent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgent_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetLRBChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows_core::HRESULT,
    pub ProcessLRB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMMediaProps(::windows_core::IUnknown);
impl IWMMediaProps {
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
}
impl ::core::convert::From<IWMMediaProps> for ::windows_core::IUnknown {
    fn from(value: IWMMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMediaProps> for ::windows_core::IUnknown {
    fn from(value: &IWMMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMediaProps {}
impl ::core::fmt::Debug for IWMMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMMediaProps {
    type Vtable = IWMMediaProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaProps_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMMetadataEditor(::windows_core::IUnknown);
impl IWMMetadataEditor {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor> for ::windows_core::IUnknown {
    fn from(value: IWMMetadataEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor> for ::windows_core::IUnknown {
    fn from(value: &IWMMetadataEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMMetadataEditor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMMetadataEditor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor {}
impl ::core::fmt::Debug for IWMMetadataEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMMetadataEditor {
    type Vtable = IWMMetadataEditor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMMetadataEditor2(::windows_core::IUnknown);
impl IWMMetadataEditor2 {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenEx)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for ::windows_core::IUnknown {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for ::windows_core::IUnknown {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMMetadataEditor2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMetadataEditor> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMetadataEditor> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMetadataEditor> for &'a IWMMetadataEditor2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMetadataEditor> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor2 {}
impl ::core::fmt::Debug for IWMMetadataEditor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMMetadataEditor2 {
    type Vtable = IWMMetadataEditor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2_Vtbl {
    pub base__: IWMMetadataEditor_Vtbl,
    pub OpenEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMMutualExclusion(::windows_core::IUnknown);
impl IWMMutualExclusion {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetStreams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion> for ::windows_core::IUnknown {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for ::windows_core::IUnknown {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMMutualExclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMMutualExclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion> for IWMStreamList {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for IWMStreamList {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for &'a IWMMutualExclusion {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion {}
impl ::core::fmt::Debug for IWMMutualExclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMMutualExclusion {
    type Vtable = IWMMutualExclusion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion_Vtbl {
    pub base__: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMMutualExclusion2(::windows_core::IUnknown);
impl IWMMutualExclusion2 {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetStreams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidtype)).ok()
    }
    pub unsafe fn GetName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetRecordCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn AddRecord(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRecord)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber)).ok()
    }
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: ::windows_core::PWSTR, pcchrecordname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRecordName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwszrecordname), ::core::mem::transmute(pcchrecordname)).ok()
    }
    pub unsafe fn SetRecordName<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wrecordnumber: u16, pwszrecordname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRecordName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber), pwszrecordname.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreamsForRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStreamForRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStreamForRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for ::windows_core::IUnknown {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for ::windows_core::IUnknown {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMStreamList {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMStreamList {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamList> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamList> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMutualExclusion> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMutualExclusion> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMutualExclusion> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMutualExclusion> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion2 {}
impl ::core::fmt::Debug for IWMMutualExclusion2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMMutualExclusion2 {
    type Vtable = IWMMutualExclusion2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2_Vtbl {
    pub base__: IWMMutualExclusion_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows_core::HRESULT,
    pub AddRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows_core::HRESULT,
    pub GetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows_core::PWSTR, pcchrecordname: *mut u16) -> ::windows_core::HRESULT,
    pub SetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetStreamsForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::HRESULT,
    pub AddStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::HRESULT,
    pub RemoveStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMOutputMediaProps(::windows_core::IUnknown);
impl IWMOutputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn GetStreamGroupName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreamGroupName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConnectionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for ::windows_core::IUnknown {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for ::windows_core::IUnknown {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMOutputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for &'a IWMOutputMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMOutputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMOutputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMOutputMediaProps {}
impl ::core::fmt::Debug for IWMOutputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMOutputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMOutputMediaProps {
    type Vtable = IWMOutputMediaProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetStreamGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMPacketSize(::windows_core::IUnknown);
impl IWMPacketSize {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize> for ::windows_core::IUnknown {
    fn from(value: IWMPacketSize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize> for ::windows_core::IUnknown {
    fn from(value: &IWMPacketSize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMPacketSize {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMPacketSize {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize {}
impl ::core::fmt::Debug for IWMPacketSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMPacketSize {
    type Vtable = IWMPacketSize_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMPacketSize2(::windows_core::IUnknown);
impl IWMPacketSize2 {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
    pub unsafe fn GetMinPacketSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMinPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwminpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize2> for ::windows_core::IUnknown {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for ::windows_core::IUnknown {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMPacketSize2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMPacketSize2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMPacketSize2> for IWMPacketSize {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for IWMPacketSize {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMPacketSize> for IWMPacketSize2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMPacketSize> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMPacketSize> for &'a IWMPacketSize2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMPacketSize> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize2 {}
impl ::core::fmt::Debug for IWMPacketSize2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMPacketSize2 {
    type Vtable = IWMPacketSize2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2_Vtbl {
    pub base__: IWMPacketSize_Vtbl,
    pub GetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows_core::HRESULT,
    pub SetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMPlayerHook(::windows_core::IUnknown);
impl IWMPlayerHook {
    pub unsafe fn PreDecode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreDecode)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMPlayerHook> for ::windows_core::IUnknown {
    fn from(value: IWMPlayerHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerHook> for ::windows_core::IUnknown {
    fn from(value: &IWMPlayerHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMPlayerHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMPlayerHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerHook {}
impl ::core::fmt::Debug for IWMPlayerHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerHook").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMPlayerHook {
    type Vtable = IWMPlayerHook_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHook_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PreDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMPlayerTimestampHook(::windows_core::IUnknown);
impl IWMPlayerTimestampHook {
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> ::windows_core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        (::windows_core::Interface::vtable(self).MapTimestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rtin), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IWMPlayerTimestampHook> for ::windows_core::IUnknown {
    fn from(value: IWMPlayerTimestampHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerTimestampHook> for ::windows_core::IUnknown {
    fn from(value: &IWMPlayerTimestampHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerTimestampHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerTimestampHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerTimestampHook {}
impl ::core::fmt::Debug for IWMPlayerTimestampHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerTimestampHook").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMPlayerTimestampHook {
    type Vtable = IWMPlayerTimestampHook_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28580dda_d98e_48d0_b7ae_69e473a02825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerTimestampHook_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub MapTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfile(::windows_core::IUnknown);
impl IWMProfile {
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<WMT_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_VERSION>(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReconfigStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows_core::GUID) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateNewStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMutualExclusionCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateNewMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
}
impl ::core::convert::From<IWMProfile> for ::windows_core::IUnknown {
    fn from(value: IWMProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile> for ::windows_core::IUnknown {
    fn from(value: &IWMProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile {}
impl ::core::fmt::Debug for IWMProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfile {
    type Vtable = IWMProfile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows_core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows_core::GUID, ppconfig: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetMutualExclusionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows_core::HRESULT,
    pub GetMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfile2(::windows_core::IUnknown);
impl IWMProfile2 {
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<WMT_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_VERSION>(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ReconfigStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows_core::GUID) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateNewStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMutualExclusionCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateNewMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetProfileID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
}
impl ::core::convert::From<IWMProfile2> for ::windows_core::IUnknown {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for ::windows_core::IUnknown {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile2> for IWMProfile {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for IWMProfile {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile> for IWMProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile> for &'a IWMProfile2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile2 {}
impl ::core::fmt::Debug for IWMProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfile2 {
    type Vtable = IWMProfile2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2_Vtbl {
    pub base__: IWMProfile_Vtbl,
    pub GetProfileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfile3(::windows_core::IUnknown);
impl IWMProfile3 {
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<WMT_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_VERSION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_VERSION>(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows_core::PWSTR, pcchname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows_core::PWSTR, pcchdescription: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveStreamByNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ReconfigStream)(::windows_core::Interface::as_raw(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows_core::GUID) -> ::windows_core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateNewStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMutualExclusionCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows_core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddMutualExclusion)(::windows_core::Interface::as_raw(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows_core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateNewMutualExclusion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProfileID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetStorageFormat(&self) -> ::windows_core::Result<WMT_STORAGE_FORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STORAGE_FORMAT>::zeroed();
        (::windows_core::Interface::vtable(self).GetStorageFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STORAGE_FORMAT>(result__)
    }
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStorageFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(nstorageformat)).ok()
    }
    pub unsafe fn GetBandwidthSharingCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetBandwidthSharingCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows_core::Result<IWMBandwidthSharing> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBandwidthSharing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwbsindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMBandwidthSharing>(result__)
    }
    pub unsafe fn RemoveBandwidthSharing<'a, Param0: ::windows_core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveBandwidthSharing)(::windows_core::Interface::as_raw(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn AddBandwidthSharing<'a, Param0: ::windows_core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddBandwidthSharing)(::windows_core::Interface::as_raw(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewBandwidthSharing(&self) -> ::windows_core::Result<IWMBandwidthSharing> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateNewBandwidthSharing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMBandwidthSharing>(result__)
    }
    pub unsafe fn GetStreamPrioritization(&self) -> ::windows_core::Result<IWMStreamPrioritization> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamPrioritization)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamPrioritization>(result__)
    }
    pub unsafe fn SetStreamPrioritization<'a, Param0: ::windows_core::IntoParam<'a, IWMStreamPrioritization>>(&self, psp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamPrioritization)(::windows_core::Interface::as_raw(self), psp.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamPrioritization(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStreamPrioritization)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateNewStreamPrioritization(&self) -> ::windows_core::Result<IWMStreamPrioritization> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateNewStreamPrioritization)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMStreamPrioritization>(result__)
    }
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetExpectedPacketCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msduration), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMProfile3> for ::windows_core::IUnknown {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for ::windows_core::IUnknown {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile> for IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile2 {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile2 {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile2> for IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfile2> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfile2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile3 {}
impl ::core::fmt::Debug for IWMProfile3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfile3 {
    type Vtable = IWMProfile3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3_Vtbl {
    pub base__: IWMProfile2_Vtbl,
    pub GetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows_core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows_core::HRESULT,
    pub GetBandwidthSharingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows_core::HRESULT,
    pub GetBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psp: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetExpectedPacketCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfileManager(::windows_core::IUnknown);
impl IWMProfileManager {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateEmptyProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LoadProfileByID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprofile: Param0) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LoadProfileByData)(::windows_core::Interface::as_raw(self), pwszprofile.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn SaveProfile<'a, Param0: ::windows_core::IntoParam<'a, IWMProfile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveProfile)(::windows_core::Interface::as_raw(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSystemProfileCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).LoadSystemProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
}
impl ::core::convert::From<IWMProfileManager> for ::windows_core::IUnknown {
    fn from(value: IWMProfileManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager> for ::windows_core::IUnknown {
    fn from(value: &IWMProfileManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfileManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfileManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager {}
impl ::core::fmt::Debug for IWMProfileManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfileManager {
    type Vtable = IWMProfileManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateEmptyProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows_core::GUID, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LoadProfileByData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprofile: ::windows_core::PCWSTR, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmprofile: ::windows_core::RawPtr, pwszprofile: ::windows_core::PCWSTR, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetSystemProfileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows_core::HRESULT,
    pub LoadSystemProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfileManager2(::windows_core::IUnknown);
impl IWMProfileManager2 {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateEmptyProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LoadProfileByID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprofile: Param0) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LoadProfileByData)(::windows_core::Interface::as_raw(self), pwszprofile.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn SaveProfile<'a, Param0: ::windows_core::IntoParam<'a, IWMProfile>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SaveProfile)(::windows_core::Interface::as_raw(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSystemProfileCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows_core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LoadSystemProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSystemProfileVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSystemProfileVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwversion)).ok()
    }
}
impl ::core::convert::From<IWMProfileManager2> for ::windows_core::IUnknown {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for ::windows_core::IUnknown {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfileManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfileManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfileManager2> for IWMProfileManager {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for IWMProfileManager {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfileManager> for IWMProfileManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfileManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMProfileManager> for &'a IWMProfileManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMProfileManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager2 {}
impl ::core::fmt::Debug for IWMProfileManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfileManager2 {
    type Vtable = IWMProfileManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2_Vtbl {
    pub base__: IWMProfileManager_Vtbl,
    pub GetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows_core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProfileManagerLanguage(::windows_core::IUnknown);
impl IWMProfileManagerLanguage {
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserLanguageID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wlangid)).ok()
    }
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserLanguageID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wlangid)).ok()
    }
}
impl ::core::convert::From<IWMProfileManagerLanguage> for ::windows_core::IUnknown {
    fn from(value: IWMProfileManagerLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManagerLanguage> for ::windows_core::IUnknown {
    fn from(value: &IWMProfileManagerLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManagerLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManagerLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManagerLanguage {}
impl ::core::fmt::Debug for IWMProfileManagerLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManagerLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProfileManagerLanguage {
    type Vtable = IWMProfileManagerLanguage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguage_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows_core::HRESULT,
    pub SetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMPropertyVault(::windows_core::IUnknown);
impl IWMPropertyVault {
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn GetPropertyByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyByName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    pub unsafe fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwsize)).ok()
    }
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: ::windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pszname), ::core::mem::transmute(pdwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    pub unsafe fn CopyPropertiesFrom<'a, Param0: ::windows_core::IntoParam<'a, IWMPropertyVault>>(&self, piwmpropertyvault: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyPropertiesFrom)(::windows_core::Interface::as_raw(self), piwmpropertyvault.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMPropertyVault> for ::windows_core::IUnknown {
    fn from(value: IWMPropertyVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPropertyVault> for ::windows_core::IUnknown {
    fn from(value: &IWMPropertyVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMPropertyVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMPropertyVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPropertyVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPropertyVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPropertyVault {}
impl ::core::fmt::Debug for IWMPropertyVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPropertyVault").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMPropertyVault {
    type Vtable = IWMPropertyVault_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVault_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows_core::HRESULT,
    pub GetPropertyByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows_core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: ::windows_core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub CopyPropertiesFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMProximityDetection(::windows_core::IUnknown);
impl IWMProximityDetection {
    pub unsafe fn StartDetection<'a, Param6: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, pbregistrationmsg: &[u8], pblocaladdress: &[u8], dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param6, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartDetection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbregistrationmsg)), pbregistrationmsg.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pblocaladdress)), pblocaladdress.len() as _, ::core::mem::transmute(dwextraportsallowed), ::core::mem::transmute(ppregistrationresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMProximityDetection> for ::windows_core::IUnknown {
    fn from(value: IWMProximityDetection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProximityDetection> for ::windows_core::IUnknown {
    fn from(value: &IWMProximityDetection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMProximityDetection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMProximityDetection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProximityDetection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProximityDetection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProximityDetection {}
impl ::core::fmt::Debug for IWMProximityDetection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProximityDetection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMProximityDetection {
    type Vtable = IWMProximityDetection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub StartDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReader(::windows_core::IUnknown);
impl IWMReader {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows_core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMReader> for ::windows_core::IUnknown {
    fn from(value: IWMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReader> for ::windows_core::IUnknown {
    fn from(value: &IWMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReader {}
impl ::core::fmt::Debug for IWMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReader {
    type Vtable = IWMReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReader_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAccelerator(::windows_core::IUnknown);
impl IWMReaderAccelerator {
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows_core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCodecInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(riid), ::core::mem::transmute(ppvcodecinterface)).ok()
    }
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(psubtype)).ok()
    }
}
impl ::core::convert::From<IWMReaderAccelerator> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAccelerator> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAccelerator {}
impl ::core::fmt::Debug for IWMReaderAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAccelerator {
    type Vtable = IWMReaderAccelerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAccelerator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCodecInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows_core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced(::windows_core::IUnknown);
impl IWMReaderAdvanced {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced {}
impl ::core::fmt::Debug for IWMReaderAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced {
    type Vtable = IWMReaderAdvanced_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub DeliverTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows_core::HRESULT,
    pub SetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_core::HRESULT,
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::HRESULT,
    pub SetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows_core::HRESULT,
    pub NotifyLateDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced2(::windows_core::IUnknown);
impl IWMReaderAdvanced2 {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PLAY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).GetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBufferProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSaveAsProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveFileAs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveFileAs)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartAtMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Preroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    pub unsafe fn SetLogClientID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogClientID)(::windows_core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetLogClientID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopBuffering)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced2 {}
impl ::core::fmt::Debug for IWMReaderAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced2 {
    type Vtable = IWMReaderAdvanced2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2_Vtbl {
    pub base__: IWMReaderAdvanced_Vtbl,
    pub SetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows_core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows_core::HRESULT,
    pub GetBufferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::HRESULT,
    pub GetDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::HRESULT,
    pub GetSaveAsProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows_core::HRESULT,
    pub SaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::HRESULT,
    pub StartAtMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
    pub Preroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::HRESULT,
    pub SetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub StopBuffering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows_core::RawPtr, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced3(::windows_core::IUnknown);
impl IWMReaderAdvanced3 {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PLAY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBufferProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSaveAsProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveFileAs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SaveFileAs)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartAtMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Preroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    pub unsafe fn SetLogClientID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLogClientID)(::windows_core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLogClientID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopBuffering)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopNetStreaming)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartAtPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced3 {}
impl ::core::fmt::Debug for IWMReaderAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced3 {
    type Vtable = IWMReaderAdvanced3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3_Vtbl {
    pub base__: IWMReaderAdvanced2_Vtbl,
    pub StopNetStreaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced4(::windows_core::IUnknown);
impl IWMReaderAdvanced4 {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PLAY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSaveAsProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveFileAs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SaveFileAs)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.StartAtMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Preroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    pub unsafe fn SetLogClientID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLogClientID)(::windows_core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLogClientID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.StopBuffering)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopNetStreaming)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartAtPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxSpeedFactor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn IsUsingFastCache(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsUsingFastCache)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AddLogParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddLogParam)(::windows_core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendLogParams)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanSaveFileAs(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CanSaveFileAs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelSaveFileAs)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced4 {}
impl ::core::fmt::Debug for IWMReaderAdvanced4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced4").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced4 {
    type Vtable = IWMReaderAdvanced4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4_Vtbl {
    pub base__: IWMReaderAdvanced3_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows_core::HRESULT,
    pub IsUsingFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub AddLogParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: ::windows_core::PCWSTR, wszname: ::windows_core::PCWSTR, wszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CanSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub CancelSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced5(::windows_core::IUnknown);
impl IWMReaderAdvanced5 {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PLAY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetBufferProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetSaveAsProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveFileAs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SaveFileAs)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.StartAtMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Preroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    pub unsafe fn SetLogClientID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetLogClientID)(::windows_core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetLogClientID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.StopBuffering)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.StopNetStreaming)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.StartAtPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxSpeedFactor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn IsUsingFastCache(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsUsingFastCache)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AddLogParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddLogParam)(::windows_core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendLogParams)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanSaveFileAs(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanSaveFileAs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelSaveFileAs)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows_core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayerHook)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced4> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced4> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced4> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced5 {}
impl ::core::fmt::Debug for IWMReaderAdvanced5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced5").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced5 {
    type Vtable = IWMReaderAdvanced5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5_Vtbl {
    pub base__: IWMReaderAdvanced4_Vtbl,
    pub SetPlayerHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAdvanced6(::windows_core::IUnknown);
impl IWMReaderAdvanced6 {
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetUserProvidedClock)(::windows_core::Interface::as_raw(self), fuserclock.into_param().abi()).ok()
    }
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetUserProvidedClock)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DeliverTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnstime)).ok()
    }
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fselection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetManualStreamSelection)(::windows_core::Interface::as_raw(self), fselection.into_param().abi()).ok()
    }
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetManualStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), fgetcallbacks.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetReceiveStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstatistics)).ok()
    }
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetClientInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.NotifyLateDelivery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows_core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PLAY_MODE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetPlayMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetBufferProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetSaveAsProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SaveFileAs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SaveFileAs)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows_core::PWSTR, pcchprotocol: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.StartAtMarker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Preroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    pub unsafe fn SetLogClientID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetLogClientID)(::windows_core::Interface::as_raw(self), flogclientid.into_param().abi()).ok()
    }
    pub unsafe fn GetLogClientID(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.GetLogClientID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.StopBuffering)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>, Param1: ::windows_core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.StopNetStreaming)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.StartAtPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetLanguageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetMaxSpeedFactor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn IsUsingFastCache(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsUsingFastCache)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AddLogParam<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddLogParam)(::windows_core::Interface::as_raw(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendLogParams)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanSaveFileAs(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanSaveFileAs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CancelSaveFileAs)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows_core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPlayerHook)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: &[u8], dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtectStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, ::core::mem::transmute(dwcertificatetype), ::core::mem::transmute(dwflags), ::core::mem::transmute(pbinitializationvector), ::core::mem::transmute(pcbinitializationvector)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced4> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced4> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced4> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced5> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced5> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderAdvanced5> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderAdvanced5> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced6 {}
impl ::core::fmt::Debug for IWMReaderAdvanced6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced6").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAdvanced6 {
    type Vtable = IWMReaderAdvanced6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6_Vtbl {
    pub base__: IWMReaderAdvanced5_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderAllocatorEx(::windows_core::IUnknown);
impl IWMReaderAllocatorEx {
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocateForStreamEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocateForOutputEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAllocatorEx> for ::windows_core::IUnknown {
    fn from(value: IWMReaderAllocatorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAllocatorEx> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderAllocatorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAllocatorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAllocatorEx {}
impl ::core::fmt::Debug for IWMReaderAllocatorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAllocatorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderAllocatorEx {
    type Vtable = IWMReaderAllocatorEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AllocateForStreamEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows_core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllocateForOutputEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows_core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderCallback(::windows_core::IUnknown);
impl IWMReaderCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnSample<'a, Param4: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallback> for ::windows_core::IUnknown {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderCallback> for IWMStatusCallback {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for IWMStatusCallback {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStatusCallback> for IWMReaderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStatusCallback> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStatusCallback> for &'a IWMReaderCallback {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStatusCallback> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallback {}
impl ::core::fmt::Debug for IWMReaderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderCallback {
    type Vtable = IWMReaderCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderCallbackAdvanced(::windows_core::IUnknown);
impl IWMReaderCallbackAdvanced {
    pub unsafe fn OnStreamSample<'a, Param4: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStreamSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnscurrenttime), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStreamSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamcount), ::core::mem::transmute(pstreamnumbers), ::core::mem::transmute(pselections), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnOutputPropsChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(pmediatype), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallbackAdvanced> for ::windows_core::IUnknown {
    fn from(value: IWMReaderCallbackAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallbackAdvanced> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderCallbackAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallbackAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallbackAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallbackAdvanced {}
impl ::core::fmt::Debug for IWMReaderCallbackAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallbackAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderCallbackAdvanced {
    type Vtable = IWMReaderCallbackAdvanced_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvanced_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnOutputPropsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderNetworkConfig(::windows_core::IUnknown);
impl IWMReaderNetworkConfig {
    pub unsafe fn GetBufferingTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetBufferingTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBufferingTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUDPPortRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUDPPortRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(prangearray)), prangearray.len() as _).ok()
    }
    pub unsafe fn GetProxySettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<WMT_PROXY_SETTINGS> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PROXY_SETTINGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetProxySettings)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    pub unsafe fn SetProxySettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxySettings)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: ::windows_core::PWSTR, pcchhostname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProxyHostName)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxyHostName)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyPort<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProxyPort)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxyPort<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxyPort)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: ::windows_core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProxyExceptionList)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxyExceptionList)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetProxyBypassForLocal)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProxyBypassForLocal)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetForceRerunAutoProxyDetection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetForceRerunAutoProxyDetection)(::windows_core::Interface::as_raw(self), fforcererundetection.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableMulticast(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableMulticast)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableMulticast)(::windows_core::Interface::as_raw(self), fenablemulticast.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableHTTP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableHTTP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableHTTP)(::windows_core::Interface::as_raw(self), fenablehttp.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableUDP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableUDP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableUDP)(::windows_core::Interface::as_raw(self), fenableudp.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableTCP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableTCP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableTCP)(::windows_core::Interface::as_raw(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetProtocolRollover)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetConnectionBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectionBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetNumProtocolsSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSupportedProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddLoggingUrl)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLoggingUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLoggingUrlCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetLoggingUrlList)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig> for ::windows_core::IUnknown {
    fn from(value: IWMReaderNetworkConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderNetworkConfig {
    type Vtable = IWMReaderNetworkConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows_core::HRESULT,
    pub SetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows_core::HRESULT,
    pub GetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_core::HRESULT,
    pub SetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows_core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows_core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_core::HRESULT,
    pub GetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszhostname: ::windows_core::PWSTR, pcchhostname: *mut u32) -> ::windows_core::HRESULT,
    pub SetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszhostname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pdwport: *mut u32) -> ::windows_core::HRESULT,
    pub SetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, dwport: u32) -> ::windows_core::HRESULT,
    pub GetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszexceptionlist: ::windows_core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_core::HRESULT,
    pub SetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pwszexceptionlist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, pfbypassforlocal: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows_core::PCWSTR, fbypassforlocal: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforcererundetection: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablemulticast: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablehttp: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablehttp: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableudp: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableudp: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabletcp: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabletcp: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ResetProtocolRollover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows_core::HRESULT,
    pub SetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows_core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows_core::HRESULT,
    pub GetSupportedProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_core::HRESULT,
    pub AddLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT,
    pub GetLoggingUrlCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows_core::HRESULT,
    pub ResetLoggingUrlList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderNetworkConfig2(::windows_core::IUnknown);
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetBufferingTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBufferingTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBufferingTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetUDPPortRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUDPPortRanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(prangearray)), prangearray.len() as _).ok()
    }
    pub unsafe fn GetProxySettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<WMT_PROXY_SETTINGS> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_PROXY_SETTINGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProxySettings)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    pub unsafe fn SetProxySettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProxySettings)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: ::windows_core::PWSTR, pcchhostname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProxyHostName)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProxyHostName)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyPort<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProxyPort)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxyPort<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProxyPort)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: ::windows_core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProxyExceptionList)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProxyExceptionList)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProxyBypassForLocal)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProxyBypassForLocal)(::windows_core::Interface::as_raw(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetForceRerunAutoProxyDetection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetForceRerunAutoProxyDetection)(::windows_core::Interface::as_raw(self), fforcererundetection.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableMulticast(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnableMulticast)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnableMulticast)(::windows_core::Interface::as_raw(self), fenablemulticast.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableHTTP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnableHTTP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnableHTTP)(::windows_core::Interface::as_raw(self), fenablehttp.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableUDP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnableUDP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnableUDP)(::windows_core::Interface::as_raw(self), fenableudp.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableTCP(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnableTCP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnableTCP)(::windows_core::Interface::as_raw(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResetProtocolRollover)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetConnectionBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetConnectionBandwidth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNumProtocolsSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows_core::PWSTR, pcchprotocolname: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSupportedProtocolName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddLoggingUrl)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetLoggingUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLoggingUrlCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResetLoggingUrlList)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetEnableContentCaching(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableContentCaching)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableContentCaching<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablecontentcaching: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableContentCaching)(::windows_core::Interface::as_raw(self), fenablecontentcaching.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableFastCache(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableFastCache)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableFastCache<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablefastcache: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableFastCache)(::windows_core::Interface::as_raw(self), fenablefastcache.into_param().abi()).ok()
    }
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetAcceleratedStreamingDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAcceleratedStreamingDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsaccelduration)).ok()
    }
    pub unsafe fn GetAutoReconnectLimit(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAutoReconnectLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoReconnectLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwautoreconnectlimit)).ok()
    }
    pub unsafe fn GetEnableResends(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableResends)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableResends<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenableresends: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableResends)(::windows_core::Interface::as_raw(self), fenableresends.into_param().abi()).ok()
    }
    pub unsafe fn GetEnableThinning(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnableThinning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetEnableThinning<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fenablethinning: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableThinning)(::windows_core::Interface::as_raw(self), fenablethinning.into_param().abi()).ok()
    }
    pub unsafe fn GetMaxNetPacketSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxNetPacketSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for ::windows_core::IUnknown {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderNetworkConfig> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderNetworkConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMReaderNetworkConfig> for &'a IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMReaderNetworkConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig2 {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderNetworkConfig2 {
    type Vtable = IWMReaderNetworkConfig2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2_Vtbl {
    pub base__: IWMReaderNetworkConfig_Vtbl,
    pub GetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablecontentcaching: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablefastcache: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows_core::HRESULT,
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows_core::HRESULT,
    pub GetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows_core::HRESULT,
    pub SetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows_core::HRESULT,
    pub GetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableresends: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableresends: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablethinning: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablethinning: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetMaxNetPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderPlaylistBurn(::windows_core::IUnknown);
impl IWMReaderPlaylistBurn {
    pub unsafe fn InitPlaylistBurn<'a, Param2: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, cfiles: u32, ppwszfilenames: *const ::windows_core::PWSTR, pcallback: Param2, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitPlaylistBurn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(ppwszfilenames), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetInitResults)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndPlaylistBurn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrburnresult)).ok()
    }
}
impl ::core::convert::From<IWMReaderPlaylistBurn> for ::windows_core::IUnknown {
    fn from(value: IWMReaderPlaylistBurn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderPlaylistBurn> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderPlaylistBurn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderPlaylistBurn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderPlaylistBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderPlaylistBurn {}
impl ::core::fmt::Debug for IWMReaderPlaylistBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderPlaylistBurn").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderPlaylistBurn {
    type Vtable = IWMReaderPlaylistBurn_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurn_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InitPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const ::windows_core::PWSTR, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetInitResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrburnresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderStreamClock(::windows_core::IUnknown);
impl IWMReaderStreamClock {
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pcnsnow)).ok()
    }
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).SetTimer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnswhen), ::core::mem::transmute(pvparam), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).KillTimer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwtimerid)).ok()
    }
}
impl ::core::convert::From<IWMReaderStreamClock> for ::windows_core::IUnknown {
    fn from(value: IWMReaderStreamClock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderStreamClock> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderStreamClock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderStreamClock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderStreamClock {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderStreamClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderStreamClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderStreamClock {}
impl ::core::fmt::Debug for IWMReaderStreamClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderStreamClock").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderStreamClock {
    type Vtable = IWMReaderStreamClock_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClock_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows_core::HRESULT,
    pub SetTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows_core::HRESULT,
    pub KillTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderTimecode(::windows_core::IUnknown);
impl IWMReaderTimecode {
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetTimecodeRangeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTimecodeRangeBounds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(wrangenum), ::core::mem::transmute(pstarttimecode), ::core::mem::transmute(pendtimecode)).ok()
    }
}
impl ::core::convert::From<IWMReaderTimecode> for ::windows_core::IUnknown {
    fn from(value: IWMReaderTimecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTimecode> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderTimecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderTimecode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderTimecode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTimecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTimecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTimecode {}
impl ::core::fmt::Debug for IWMReaderTimecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTimecode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderTimecode {
    type Vtable = IWMReaderTimecode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecode_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTimecodeRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows_core::HRESULT,
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMReaderTypeNegotiation(::windows_core::IUnknown);
impl IWMReaderTypeNegotiation {
    pub unsafe fn TryOutputProps<'a, Param1: ::windows_core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TryOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderTypeNegotiation> for ::windows_core::IUnknown {
    fn from(value: IWMReaderTypeNegotiation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTypeNegotiation> for ::windows_core::IUnknown {
    fn from(value: &IWMReaderTypeNegotiation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTypeNegotiation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTypeNegotiation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTypeNegotiation {}
impl ::core::fmt::Debug for IWMReaderTypeNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTypeNegotiation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMReaderTypeNegotiation {
    type Vtable = IWMReaderTypeNegotiation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TryOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMRegisterCallback(::windows_core::IUnknown);
impl IWMRegisterCallback {
    pub unsafe fn Advise<'a, Param0: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Advise)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows_core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unadvise)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMRegisterCallback> for ::windows_core::IUnknown {
    fn from(value: IWMRegisterCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisterCallback> for ::windows_core::IUnknown {
    fn from(value: &IWMRegisterCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMRegisterCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMRegisterCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisterCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisterCallback {}
impl ::core::fmt::Debug for IWMRegisterCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisterCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMRegisterCallback {
    type Vtable = IWMRegisterCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMRegisteredDevice(::windows_core::IUnknown);
impl IWMRegisteredDevice {
    pub unsafe fn GetDeviceSerialNumber(&self) -> ::windows_core::Result<DRM_VAL16> {
        let mut result__ = ::core::mem::MaybeUninit::<DRM_VAL16>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceSerialNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DRM_VAL16>(result__)
    }
    pub unsafe fn GetDeviceCertificate(&self) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn GetDeviceType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributeCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut ::win32_foundation::BSTR, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributeByIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrvalue)).ok()
    }
    pub unsafe fn GetAttributeByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributeByName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAttributeByName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttributeByName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    pub unsafe fn Approve<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fapprove: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Approve)(::windows_core::Interface::as_raw(self), fapprove.into_param().abi()).ok()
    }
    pub unsafe fn IsValid(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsValid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn IsApproved(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsApproved)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn IsWmdrmCompliant(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsWmdrmCompliant)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn IsOpened(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsOpened)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn Open(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMRegisteredDevice> for ::windows_core::IUnknown {
    fn from(value: IWMRegisteredDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisteredDevice> for ::windows_core::IUnknown {
    fn from(value: &IWMRegisteredDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMRegisteredDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMRegisteredDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisteredDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisteredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisteredDevice {}
impl ::core::fmt::Debug for IWMRegisteredDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisteredDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMRegisteredDevice {
    type Vtable = IWMRegisteredDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDevice_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDeviceSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows_core::HRESULT,
    pub GetDeviceCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows_core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut ::win32_foundation::BSTR, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Approve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fapprove: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsApproved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfapproved: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsWmdrmCompliant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompliant: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfopened: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSBufferAllocator(::windows_core::IUnknown);
impl IWMSBufferAllocator {
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AllocateBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AllocatePageSizeBuffer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
}
impl ::core::convert::From<IWMSBufferAllocator> for ::windows_core::IUnknown {
    fn from(value: IWMSBufferAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSBufferAllocator> for ::windows_core::IUnknown {
    fn from(value: &IWMSBufferAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSBufferAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSBufferAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSBufferAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSBufferAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSBufferAllocator {}
impl ::core::fmt::Debug for IWMSBufferAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSBufferAllocator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSBufferAllocator {
    type Vtable = IWMSBufferAllocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource(::windows_core::IUnknown);
impl IWMSInternalAdminNetSource {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, INSNetSourceCreator>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, psharednamespace: Param0, pnamespacenode: Param1, pnetsourcecreator: Param2, fembeddedinserver: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), psharednamespace.into_param().abi(), pnamespacenode.into_param().abi(), pnetsourcecreator.into_param().abi(), fembeddedinserver.into_param().abi()).ok()
    }
    pub unsafe fn GetNetSourceCreator(&self) -> ::windows_core::Result<INSNetSourceCreator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetSourceCreator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSNetSourceCreator>(result__)
    }
    pub unsafe fn SetCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrname: Param1, bstrpassword: Param2, fpersist: Param3, fconfirmedgood: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrealm: Param0, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCredentials)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    pub unsafe fn DeleteCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteCredentials)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetCredentialFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCredentialFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn FindProxyForURL<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindProxyForURL)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    pub unsafe fn RegisterProxyFailure(&self, hrparam: ::windows_core::HRESULT, dwproxycontext: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterProxyFailure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(dwproxycontext)).ok()
    }
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownProxyContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwproxycontext)).ok()
    }
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsUsingIE)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwproxycontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource> for ::windows_core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource> for ::windows_core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSInternalAdminNetSource {
    type Vtable = IWMSInternalAdminNetSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bb23e5f_d127_4afb_8d02_ae5b66d54c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows_core::RawPtr, fembeddedinserver: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetNetSourceCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpersist: ::win32_foundation::BOOL, fconfirmedgood: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub DeleteCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub SetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub FindProxyForURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::HRESULT,
    pub RegisterProxyFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows_core::HRESULT, dwproxycontext: u32) -> ::windows_core::HRESULT,
    pub ShutdownProxyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows_core::HRESULT,
    pub IsUsingIE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource2(::windows_core::IUnknown);
impl IWMSInternalAdminNetSource2 {
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindProxyForURLEx)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource2> for ::windows_core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource2> for ::windows_core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource2 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSInternalAdminNetSource2 {
    type Vtable = IWMSInternalAdminNetSource2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe74d58c3_cf77_4b51_af17_744687c43eae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fproxy: ::win32_foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpersist: ::win32_foundation::BOOL, fconfirmedgood: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fproxy: ::win32_foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub DeleteCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fproxy: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub FindProxyForURLEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource3(::windows_core::IUnknown);
impl IWMSInternalAdminNetSource3 {
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteCredentialsEx)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FindProxyForURLEx)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    pub unsafe fn GetNetSourceCreator2(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetSourceCreator2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn FindProxyForURLEx2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindProxyForURLEx2)(::windows_core::Interface::as_raw(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pqwproxycontext)).ok()
    }
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: ::windows_core::HRESULT, qwproxycontext: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterProxyFailure2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(qwproxycontext)).ok()
    }
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownProxyContext2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(qwproxycontext)).ok()
    }
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsUsingIE2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(qwproxycontext), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetCredentialsEx2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6, fcleartextauthentication: Param7) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCredentialsEx2)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi(), fcleartextauthentication.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialsEx2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, fcleartextauthentication: Param3, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCredentialsEx2)(::windows_core::Interface::as_raw(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), fcleartextauthentication.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for ::windows_core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for ::windows_core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMSInternalAdminNetSource2> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMSInternalAdminNetSource2> for &'a IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource3 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSInternalAdminNetSource3 {
    type Vtable = IWMSInternalAdminNetSource3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b63d08e_4590_44af_9eb3_57ff1e73bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource3_Vtbl {
    pub base__: IWMSInternalAdminNetSource2_Vtbl,
    pub GetNetSourceCreator2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindProxyForURLEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfproxyenabled: *mut ::win32_foundation::BOOL, pbstrproxyserver: *mut ::win32_foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows_core::HRESULT,
    pub RegisterProxyFailure2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows_core::HRESULT, qwproxycontext: u64) -> ::windows_core::HRESULT,
    pub ShutdownProxyContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows_core::HRESULT,
    pub IsUsingIE2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fproxy: ::win32_foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fpersist: ::win32_foundation::BOOL, fconfirmedgood: ::win32_foundation::BOOL, fcleartextauthentication: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fproxy: ::win32_foundation::BOOL, fcleartextauthentication: ::win32_foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::win32_foundation::BSTR, pbstrpassword: *mut ::win32_foundation::BSTR, pfconfirmedgood: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSecureChannel(::windows_core::IUnknown);
impl IWMSecureChannel {
    pub unsafe fn GetCertCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCertCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows_core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut u8>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSharedData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn WMSC_AddCertificate<'a, Param0: ::windows_core::IntoParam<'a, IWMAuthorizer>>(&self, pcert: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_AddCertificate)(::windows_core::Interface::as_raw(self), pcert.into_param().abi()).ok()
    }
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_AddSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbcertsig), ::core::mem::transmute(cbcertsig)).ok()
    }
    pub unsafe fn WMSC_Connect<'a, Param0: ::windows_core::IntoParam<'a, IWMSecureChannel>>(&self, potherside: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Connect)(::windows_core::Interface::as_raw(self), potherside.into_param().abi()).ok()
    }
    pub unsafe fn WMSC_IsConnected(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).WMSC_IsConnected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn WMSC_Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_GetValidCertificate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppbcertificate), ::core::mem::transmute(pdwsignature)).ok()
    }
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Encrypt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Decrypt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    pub unsafe fn WMSC_Lock(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Lock)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_Unlock(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_Unlock)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WMSC_SetSharedData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata)).ok()
    }
}
impl ::core::convert::From<IWMSecureChannel> for ::windows_core::IUnknown {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for ::windows_core::IUnknown {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSecureChannel> for IWMAuthorizer {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for IWMAuthorizer {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMAuthorizer> for IWMSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, IWMAuthorizer> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMAuthorizer> for &'a IWMSecureChannel {
    fn into_param(self) -> ::windows_core::Param<'a, IWMAuthorizer> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSecureChannel {}
impl ::core::fmt::Debug for IWMSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSecureChannel {
    type Vtable = IWMSecureChannel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2720598a_d0f2_4189_bd10_91c46ef0936f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSecureChannel_Vtbl {
    pub base__: IWMAuthorizer_Vtbl,
    pub WMSC_AddCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcert: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WMSC_AddSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows_core::HRESULT,
    pub WMSC_Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, potherside: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WMSC_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub WMSC_Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WMSC_GetValidCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows_core::HRESULT,
    pub WMSC_Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT,
    pub WMSC_Decrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT,
    pub WMSC_Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WMSC_Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WMSC_SetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStatusCallback(::windows_core::IUnknown);
impl IWMStatusCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMStatusCallback> for ::windows_core::IUnknown {
    fn from(value: IWMStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStatusCallback> for ::windows_core::IUnknown {
    fn from(value: &IWMStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStatusCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStatusCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStatusCallback {}
impl ::core::fmt::Debug for IWMStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStatusCallback {
    type Vtable = IWMStatusCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStreamConfig(::windows_core::IUnknown);
impl IWMStreamConfig {
    pub unsafe fn GetStreamType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreamName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    pub unsafe fn SetStreamName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamName)(::windows_core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConnectionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    pub unsafe fn SetConnectionName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConnectionName)(::windows_core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig> for ::windows_core::IUnknown {
    fn from(value: IWMStreamConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig> for ::windows_core::IUnknown {
    fn from(value: &IWMStreamConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStreamConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStreamConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig {}
impl ::core::fmt::Debug for IWMStreamConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStreamConfig {
    type Vtable = IWMStreamConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStreamType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows_core::HRESULT,
    pub SetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT,
    pub GetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::HRESULT,
    pub SetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::HRESULT,
    pub SetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows_core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows_core::HRESULT,
    pub GetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows_core::HRESULT,
    pub SetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStreamConfig2(::windows_core::IUnknown);
impl IWMStreamConfig2 {
    pub unsafe fn GetStreamType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetStreamName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    pub unsafe fn SetStreamName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStreamName)(::windows_core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConnectionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    pub unsafe fn SetConnectionName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetConnectionName)(::windows_core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows_core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_TRANSPORT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransportType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransportType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDataUnitExtension)(::windows_core::Interface::as_raw(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbextensionsysteminfo)), pbextensionsysteminfo.len() as _).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetDataUnitExtensionCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDataUnitExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAllDataUnitExtensions)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig2> for ::windows_core::IUnknown {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for ::windows_core::IUnknown {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStreamConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig> for &'a IWMStreamConfig2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig2 {}
impl ::core::fmt::Debug for IWMStreamConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStreamConfig2 {
    type Vtable = IWMStreamConfig2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2_Vtbl {
    pub base__: IWMStreamConfig_Vtbl,
    pub GetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows_core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_core::HRESULT,
    pub AddDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows_core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows_core::HRESULT,
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows_core::HRESULT,
    pub GetDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStreamConfig3(::windows_core::IUnknown);
impl IWMStreamConfig3 {
    pub unsafe fn GetStreamType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetStreamNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows_core::PWSTR, pcchstreamname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetStreamName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    pub unsafe fn SetStreamName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetStreamName)(::windows_core::Interface::as_raw(self), pwszstreamname.into_param().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows_core::PWSTR, pcchinputname: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetConnectionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    pub unsafe fn SetConnectionName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetConnectionName)(::windows_core::Interface::as_raw(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBitrate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBufferWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows_core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_TRANSPORT_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransportType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTransportType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDataUnitExtension)(::windows_core::Interface::as_raw(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(::windows_core::as_ptr_or_null(pbextensionsysteminfo)), pbextensionsysteminfo.len() as _).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDataUnitExtensionCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows_core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDataUnitExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveAllDataUnitExtensions)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLanguage)(::windows_core::Interface::as_raw(self), pwszlanguagestring.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig3> for ::windows_core::IUnknown {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for ::windows_core::IUnknown {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig2> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStreamConfig2> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStreamConfig2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig3 {}
impl ::core::fmt::Debug for IWMStreamConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStreamConfig3 {
    type Vtable = IWMStreamConfig3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3_Vtbl {
    pub base__: IWMStreamConfig2_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStreamList(::windows_core::IUnknown);
impl IWMStreamList {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStreams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum)).ok()
    }
}
impl ::core::convert::From<IWMStreamList> for ::windows_core::IUnknown {
    fn from(value: IWMStreamList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamList> for ::windows_core::IUnknown {
    fn from(value: &IWMStreamList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStreamList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStreamList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamList {}
impl ::core::fmt::Debug for IWMStreamList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStreamList {
    type Vtable = IWMStreamList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows_core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMStreamPrioritization(::windows_core::IUnknown);
impl IWMStreamPrioritization {
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPriorityRecords)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(pcrecords)).ok()
    }
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriorityRecords)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(crecords)).ok()
    }
}
impl ::core::convert::From<IWMStreamPrioritization> for ::windows_core::IUnknown {
    fn from(value: IWMStreamPrioritization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamPrioritization> for ::windows_core::IUnknown {
    fn from(value: &IWMStreamPrioritization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMStreamPrioritization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMStreamPrioritization {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamPrioritization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamPrioritization {}
impl ::core::fmt::Debug for IWMStreamPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamPrioritization").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMStreamPrioritization {
    type Vtable = IWMStreamPrioritization_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritization_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows_core::HRESULT,
    pub SetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMSyncReader(::windows_core::IUnknown);
impl IWMSyncReader {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRangeByFrame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNextSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReadStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetReadStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows_core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputNumberForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetStreamNumberForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, pstream: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMSyncReader> for ::windows_core::IUnknown {
    fn from(value: IWMSyncReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader> for ::windows_core::IUnknown {
    fn from(value: &IWMSyncReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSyncReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSyncReader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader {}
impl ::core::fmt::Debug for IWMSyncReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSyncReader {
    type Vtable = IWMSyncReader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows_core::HRESULT,
    pub SetRangeByFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::HRESULT,
    pub GetNextSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows_core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows_core::HRESULT,
    pub SetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetOutputNumberForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows_core::HRESULT,
    pub GetStreamNumberForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows_core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[repr(transparent)]
pub struct IWMSyncReader2(::windows_core::IUnknown);
impl IWMSyncReader2 {
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRange)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRangeByFrame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetNextSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStreamsSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows_core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_STREAM_SELECTION>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamSelected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReadStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetReadStreamSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOutputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutputCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows_core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOutputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutputFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows_core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutputFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutputNumberForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStreamNumberForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxOutputSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxStreamSampleSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstream), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(&self, pstream: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OpenStream)(::windows_core::Interface::as_raw(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRangeByTimecode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pstart), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).SetRangeByFrameEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows_core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, dwoutputnum: u32, pallocator: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows_core::Result<IWMReaderAllocatorEx> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllocateForOutput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMReaderAllocatorEx>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows_core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, wstreamnum: u16, pallocator: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows_core::Result<IWMReaderAllocatorEx> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllocateForStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMReaderAllocatorEx>(result__)
    }
}
impl ::core::convert::From<IWMSyncReader2> for ::windows_core::IUnknown {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for ::windows_core::IUnknown {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMSyncReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMSyncReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSyncReader2> for IWMSyncReader {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for IWMSyncReader {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMSyncReader> for IWMSyncReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMSyncReader> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMSyncReader> for &'a IWMSyncReader2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMSyncReader> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader2 {}
impl ::core::fmt::Debug for IWMSyncReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMSyncReader2 {
    type Vtable = IWMSyncReader2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2_Vtbl {
    pub base__: IWMSyncReader_Vtbl,
    pub SetRangeByTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows_core::HRESULT,
    pub SetRangeByFrameEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows_core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMVideoMediaProps(::windows_core::IUnknown);
impl IWMVideoMediaProps {
    pub unsafe fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMediaType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxKeyFrameSpacing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxKeyFrameSpacing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lltime)).ok()
    }
    pub unsafe fn GetQuality(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetQuality)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQuality(&self, dwquality: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuality)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwquality)).ok()
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for ::windows_core::IUnknown {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for ::windows_core::IUnknown {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMVideoMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMMediaProps> for &'a IWMVideoMediaProps {
    fn into_param(self) -> ::windows_core::Param<'a, IWMMediaProps> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMVideoMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMVideoMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoMediaProps {}
impl ::core::fmt::Debug for IWMVideoMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMVideoMediaProps {
    type Vtable = IWMVideoMediaProps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaProps_Vtbl {
    pub base__: IWMMediaProps_Vtbl,
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows_core::HRESULT,
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows_core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWatermarkInfo(::windows_core::IUnknown);
impl IWMWatermarkInfo {
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetWatermarkEntryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows_core::Result<WMT_WATERMARK_ENTRY> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_WATERMARK_ENTRY>::zeroed();
        (::windows_core::Interface::vtable(self).GetWatermarkEntry)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_WATERMARK_ENTRY>(result__)
    }
}
impl ::core::convert::From<IWMWatermarkInfo> for ::windows_core::IUnknown {
    fn from(value: IWMWatermarkInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWatermarkInfo> for ::windows_core::IUnknown {
    fn from(value: &IWMWatermarkInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWatermarkInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWatermarkInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWatermarkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWatermarkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWatermarkInfo {}
impl ::core::fmt::Debug for IWMWatermarkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWatermarkInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWatermarkInfo {
    type Vtable = IWMWatermarkInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetWatermarkEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetWatermarkEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriter(::windows_core::IUnknown);
impl IWMWriter {
    pub unsafe fn SetProfileByID(&self, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProfileByID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidprofile)).ok()
    }
    pub unsafe fn SetProfile<'a, Param0: ::windows_core::IntoParam<'a, IWMProfile>>(&self, pprofile: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProfile)(::windows_core::Interface::as_raw(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputFilename)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetInputCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetInputCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> ::windows_core::Result<IWMInputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMInputMediaProps>(result__)
    }
    pub unsafe fn SetInputProps<'a, Param1: ::windows_core::IntoParam<'a, IWMInputMediaProps>>(&self, dwinputnum: u32, pinput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInputProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), pinput.into_param().abi()).ok()
    }
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetInputFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows_core::Result<IWMInputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInputFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMInputMediaProps>(result__)
    }
    pub unsafe fn BeginWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AllocateSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsamplesize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn WriteSample<'a, Param3: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriter> for ::windows_core::IUnknown {
    fn from(value: IWMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriter> for ::windows_core::IUnknown {
    fn from(value: &IWMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriter {}
impl ::core::fmt::Debug for IWMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriter {
    type Vtable = IWMWriter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows_core::HRESULT,
    pub GetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetInputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetInputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllocateSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WriteSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterAdvanced(::windows_core::IUnknown);
impl IWMWriterAdvanced {
    pub unsafe fn GetSinkCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSinkCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows_core::Result<IWMWriterSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteStreamSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn SetLiveSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLiveSource)(::windows_core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetWriterTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows_core::Result<WM_WRITER_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_WRITER_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMWriterAdvanced> for ::windows_core::IUnknown {
    fn from(value: IWMWriterAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterAdvanced {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced {}
impl ::core::fmt::Debug for IWMWriterAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterAdvanced {
    type Vtable = IWMWriterAdvanced_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetSinkCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows_core::HRESULT,
    pub GetSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WriteStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLiveSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetWriterTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows_core::HRESULT,
    pub SetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows_core::HRESULT,
    pub GetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterAdvanced2(::windows_core::IUnknown);
impl IWMWriterAdvanced2 {
    pub unsafe fn GetSinkCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSinkCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows_core::Result<IWMWriterSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.WriteStreamSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn SetLiveSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLiveSource)(::windows_core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWriterTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows_core::Result<WM_WRITER_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_WRITER_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetInputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for ::windows_core::IUnknown {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced> for &'a IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced2 {}
impl ::core::fmt::Debug for IWMWriterAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterAdvanced2 {
    type Vtable = IWMWriterAdvanced2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2_Vtbl {
    pub base__: IWMWriterAdvanced_Vtbl,
    pub GetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows_core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::HRESULT,
    pub SetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows_core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterAdvanced3(::windows_core::IUnknown);
impl IWMWriterAdvanced3 {
    pub unsafe fn GetSinkCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSinkCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows_core::Result<IWMWriterSink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RemoveSink)(::windows_core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.WriteStreamSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn SetLiveSource<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetLiveSource)(::windows_core::Interface::as_raw(self), fislivesource.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWriterTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows_core::Result<WM_WRITER_STATISTICS> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_WRITER_STATISTICS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSyncTolerance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    pub unsafe fn SetInputSetting<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInputSetting)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows_core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16) -> ::windows_core::Result<WM_WRITER_STATISTICS_EX> {
        let mut result__ = ::core::mem::MaybeUninit::<WM_WRITER_STATISTICS_EX>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatisticsEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WM_WRITER_STATISTICS_EX>(result__)
    }
    pub unsafe fn SetNonBlocking(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNonBlocking)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for ::windows_core::IUnknown {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced2> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterAdvanced2> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterAdvanced2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced3 {}
impl ::core::fmt::Debug for IWMWriterAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterAdvanced3 {
    type Vtable = IWMWriterAdvanced3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3_Vtbl {
    pub base__: IWMWriterAdvanced2_Vtbl,
    pub GetStatisticsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows_core::HRESULT,
    pub SetNonBlocking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterFileSink(::windows_core::IUnknown);
impl IWMWriterFileSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink> for ::windows_core::IUnknown {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterFileSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterFileSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink> for IWMWriterSink {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink {}
impl ::core::fmt::Debug for IWMWriterFileSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterFileSink {
    type Vtable = IWMWriterFileSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterFileSink2(::windows_core::IUnknown);
impl IWMWriterFileSink2 {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    pub unsafe fn IsStopped(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsStopped)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsClosed(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsClosed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for ::windows_core::IUnknown {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink2 {}
impl ::core::fmt::Debug for IWMWriterFileSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterFileSink2 {
    type Vtable = IWMWriterFileSink2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2_Vtbl {
    pub base__: IWMWriterFileSink_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows_core::HRESULT,
    pub IsStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfstopped: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetFileDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfclosed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterFileSink3(::windows_core::IUnknown);
impl IWMWriterFileSink3 {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Open)(::windows_core::Interface::as_raw(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Start)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stop)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    pub unsafe fn IsStopped(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsStopped)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFileDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFileSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsClosed(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsClosed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetAutoIndexing<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fdoautoindexing: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoIndexing)(::windows_core::Interface::as_raw(self), fdoautoindexing.into_param().abi()).ok()
    }
    pub unsafe fn GetAutoIndexing(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAutoIndexing)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetControlStream<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetControlStream)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), fshouldcontrolstartandstop.into_param().abi()).ok()
    }
    pub unsafe fn GetMode(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataUnitEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfilesinkdataunit)).ok()
    }
    pub unsafe fn SetUnbufferedIO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, funbufferedio: Param0, frestrictmemusage: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnbufferedIO)(::windows_core::Interface::as_raw(self), funbufferedio.into_param().abi(), frestrictmemusage.into_param().abi()).ok()
    }
    pub unsafe fn GetUnbufferedIO(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetUnbufferedIO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn CompleteOperations(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CompleteOperations)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for ::windows_core::IUnknown {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink2> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterFileSink2> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterFileSink2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink3 {}
impl ::core::fmt::Debug for IWMWriterFileSink3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterFileSink3 {
    type Vtable = IWMWriterFileSink3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3_Vtbl {
    pub base__: IWMWriterFileSink2_Vtbl,
    pub SetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdoautoindexing: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautoindexing: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetControlStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows_core::HRESULT,
    pub OnDataUnitEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows_core::HRESULT,
    pub SetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, funbufferedio: ::win32_foundation::BOOL, frestrictmemusage: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub CompleteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterNetworkSink(::windows_core::IUnknown);
impl IWMWriterNetworkSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaximumClients)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxclients)).ok()
    }
    pub unsafe fn GetMaximumClients(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaximumClients)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetworkProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(protocol)).ok()
    }
    pub unsafe fn GetNetworkProtocol(&self) -> ::windows_core::Result<WMT_NET_PROTOCOL> {
        let mut result__ = ::core::mem::MaybeUninit::<WMT_NET_PROTOCOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetNetworkProtocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WMT_NET_PROTOCOL>(result__)
    }
    pub unsafe fn GetHostURL(&self, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHostURL)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwportnum)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for ::windows_core::IUnknown {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterNetworkSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterNetworkSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterNetworkSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterNetworkSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterNetworkSink {}
impl ::core::fmt::Debug for IWMWriterNetworkSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterNetworkSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterNetworkSink {
    type Vtable = IWMWriterNetworkSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub SetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows_core::HRESULT,
    pub GetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows_core::HRESULT,
    pub SetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows_core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows_core::HRESULT,
    pub GetHostURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PWSTR, pcchurl: *mut u32) -> ::windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterPostView(::windows_core::IUnknown);
impl IWMWriterPostView {
    pub unsafe fn SetPostViewCallback<'a, Param0: ::windows_core::IntoParam<'a, IWMWriterPostViewCallback>>(&self, pcallback: Param0, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPostViewCallback)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn SetReceivePostViewSamples<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnum: u16, freceivepostviewsamples: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReceivePostViewSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), freceivepostviewsamples.into_param().abi()).ok()
    }
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetReceivePostViewSamples)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows_core::Result<IWMMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPostViewProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMediaProps>(result__)
    }
    pub unsafe fn SetPostViewProps<'a, Param1: ::windows_core::IntoParam<'a, IWMMediaProps>>(&self, wstreamnumber: u16, poutput: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPostViewProps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetPostViewFormatCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows_core::Result<IWMMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPostViewFormat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMediaProps>(result__)
    }
    pub unsafe fn SetAllocateForPostView<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, wstreamnumber: u16, fallocate: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllocateForPostView)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), fallocate.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetAllocateForPostView)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterPostView> for ::windows_core::IUnknown {
    fn from(value: IWMWriterPostView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostView> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterPostView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterPostView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterPostView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostView {}
impl ::core::fmt::Debug for IWMWriterPostView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterPostView {
    type Vtable = IWMWriterPostView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostView_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetPostViewCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows_core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPostViewFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetPostViewFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterPostViewCallback(::windows_core::IUnknown);
impl IWMWriterPostViewCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows_core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnPostViewSample<'a, Param4: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnPostViewSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocateForPostView)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for ::windows_core::IUnknown {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStatusCallback> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStatusCallback> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMStatusCallback> for &'a IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows_core::Param<'a, IWMStatusCallback> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostViewCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostViewCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostViewCallback {}
impl ::core::fmt::Debug for IWMWriterPostViewCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostViewCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterPostViewCallback {
    type Vtable = IWMWriterPostViewCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallback_Vtbl {
    pub base__: IWMStatusCallback_Vtbl,
    pub OnPostViewSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows_core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterPreprocess(::windows_core::IUnknown);
impl IWMWriterPreprocess {
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxPreprocessingPasses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNumPreprocessingPasses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwnumpasses)).ok()
    }
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPreprocessingPass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn PreprocessSample<'a, Param3: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreprocessSample)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndPreprocessingPass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IWMWriterPreprocess> for ::windows_core::IUnknown {
    fn from(value: IWMWriterPreprocess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPreprocess> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterPreprocess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterPreprocess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterPreprocess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPreprocess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPreprocess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPreprocess {}
impl ::core::fmt::Debug for IWMWriterPreprocess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPreprocess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterPreprocess {
    type Vtable = IWMWriterPreprocess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocess_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows_core::HRESULT,
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows_core::HRESULT,
    pub BeginPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows_core::HRESULT,
    pub PreprocessSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EndPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterPushSink(::windows_core::IUnknown);
impl IWMWriterPushSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pwszurl: Param0, pwsztemplateurl: Param1, fautodestroy: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), pwszurl.into_param().abi(), pwsztemplateurl.into_param().abi(), fautodestroy.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndSession(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterPushSink> for ::windows_core::IUnknown {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterPushSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterPushSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPushSink> for IWMWriterSink {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for IWMWriterSink {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for IWMWriterPushSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterPushSink {
    fn into_param(self) -> ::windows_core::Param<'a, IWMWriterSink> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPushSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPushSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPushSink {}
impl ::core::fmt::Debug for IWMWriterPushSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPushSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterPushSink {
    type Vtable = IWMWriterPushSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSink_Vtbl {
    pub base__: IWMWriterSink_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pwsztemplateurl: ::windows_core::PCWSTR, fautodestroy: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWMWriterSink(::windows_core::IUnknown);
impl IWMWriterSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnHeader)(::windows_core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsRealTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows_core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AllocateDataUnit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows_core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataUnit)(::windows_core::Interface::as_raw(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEndWriting)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterSink> for ::windows_core::IUnknown {
    fn from(value: IWMWriterSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterSink> for ::windows_core::IUnknown {
    fn from(value: &IWMWriterSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWMWriterSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWMWriterSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterSink {}
impl ::core::fmt::Debug for IWMWriterSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IWMWriterSink {
    type Vtable = IWMWriterSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSink_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub AllocateDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnEndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(pub i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(0i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(1i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(2i32);
impl ::core::marker::Copy for NETSOURCE_URLCREDPOLICY_SETTINGS {}
impl ::core::clone::Clone for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NETSOURCE_URLCREDPOLICY_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSOURCE_URLCREDPOLICY_SETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WEBSTREAM_SAMPLE_TYPE(pub i32);
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(1i32);
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(2i32);
impl ::core::marker::Copy for WEBSTREAM_SAMPLE_TYPE {}
impl ::core::clone::Clone for WEBSTREAM_SAMPLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEBSTREAM_SAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WEBSTREAM_SAMPLE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEBSTREAM_SAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEBSTREAM_SAMPLE_TYPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn WMCreateBackupRestorer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pcallback: Param0) -> ::windows_core::Result<IWMLicenseBackup> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateBackupRestorer(pcallback: *mut ::core::ffi::c_void, ppbackup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateBackupRestorer(pcallback.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMLicenseBackup>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateEditor() -> ::windows_core::Result<IWMMetadataEditor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateEditor(ppeditor: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateEditor(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMMetadataEditor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateIndexer() -> ::windows_core::Result<IWMIndexer> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateIndexer(ppindexer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateIndexer(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMIndexer>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateProfileManager() -> ::windows_core::Result<IWMProfileManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateProfileManager(ppprofilemanager: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateProfileManager(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMProfileManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateReader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows_core::Result<IWMReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppreader: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateSyncReader<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows_core::Result<IWMSyncReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateSyncReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppsyncreader: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateSyncReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMSyncReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkcert: Param0) -> ::windows_core::Result<IWMWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriter(punkcert: *mut ::core::ffi::c_void, ppwriter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateWriter(punkcert.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> ::windows_core::Result<IWMWriterFileSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterFileSink(ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateWriterFileSink(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterFileSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> ::windows_core::Result<IWMWriterNetworkSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterNetworkSink(ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateWriterNetworkSink(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterNetworkSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> ::windows_core::Result<IWMWriterPushSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterPushSink(ppsink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        WMCreateWriterPushSink(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWMWriterPushSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl ::core::marker::Copy for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::clone::Clone for WMDRM_IMPORT_INIT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDRM_IMPORT_INIT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDRM_IMPORT_INIT_STRUCT").field("dwVersion", &self.dwVersion).field("cbEncryptedSessionKeyMessage", &self.cbEncryptedSessionKeyMessage).field("pbEncryptedSessionKeyMessage", &self.pbEncryptedSessionKeyMessage).field("cbEncryptedKeyMessage", &self.cbEncryptedKeyMessage).field("pbEncryptedKeyMessage", &self.pbEncryptedKeyMessage).finish()
    }
}
unsafe impl ::windows_core::Abi for WMDRM_IMPORT_INIT_STRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDRM_IMPORT_INIT_STRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
#[inline]
pub unsafe fn WMIsContentProtected<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwszfilename: Param0, pfisprotected: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMIsContentProtected(pwszfilename: ::windows_core::PCWSTR, pfisprotected: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        WMIsContentProtected(pwszfilename.into_param().abi(), ::core::mem::transmute(pfisprotected)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WMMEDIASUBTYPE_ACELPnet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for WMMPEG2VIDEOINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for WMMPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for WMMPEG2VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMMPEG2VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("dwProfile", &self.dwProfile).field("dwLevel", &self.dwLevel).field("dwFlags", &self.dwFlags).field("dwSequenceHeader", &self.dwSequenceHeader).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for WMMPEG2VIDEOINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMMPEG2VIDEOINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows_core::GUID,
}
impl ::core::marker::Copy for WMSCRIPTFORMAT {}
impl ::core::clone::Clone for WMSCRIPTFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMSCRIPTFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMSCRIPTFORMAT").field("scriptType", &self.scriptType).finish()
    }
}
unsafe impl ::windows_core::Abi for WMSCRIPTFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMSCRIPTFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMSCRIPTFORMAT {}
impl ::core::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMSCRIPTTYPE_TwoStrings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_ATTR_DATATYPE(pub i32);
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(0i32);
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(1i32);
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(2i32);
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(3i32);
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(4i32);
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(5i32);
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(6i32);
impl ::core::marker::Copy for WMT_ATTR_DATATYPE {}
impl ::core::clone::Clone for WMT_ATTR_DATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_ATTR_DATATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_ATTR_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_DATATYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_ATTR_IMAGETYPE(pub i32);
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(1i32);
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(2i32);
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(3i32);
impl ::core::marker::Copy for WMT_ATTR_IMAGETYPE {}
impl ::core::clone::Clone for WMT_ATTR_IMAGETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_IMAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_ATTR_IMAGETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_ATTR_IMAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_IMAGETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: ::core::option::Option<INSSBuffer>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl ::core::clone::Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        Self { pBuffer: self.pBuffer.clone(), cbOffset: self.cbOffset, cbLength: self.cbLength }
    }
}
impl ::core::fmt::Debug for WMT_BUFFER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_BUFFER_SEGMENT").field("pBuffer", &self.pBuffer).field("cbOffset", &self.cbOffset).field("cbLength", &self.cbLength).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_BUFFER_SEGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::core::cmp::Eq for WMT_BUFFER_SEGMENT {}
impl ::core::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_CODEC_INFO_TYPE(pub i32);
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(0i32);
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(1i32);
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(-1i32);
impl ::core::marker::Copy for WMT_CODEC_INFO_TYPE {}
impl ::core::clone::Clone for WMT_CODEC_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CODEC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_CODEC_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_CODEC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CODEC_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl ::core::marker::Copy for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_COLORSPACEINFO_EXTENSION_DATA").field("ucColorPrimaries", &self.ucColorPrimaries).field("ucColorTransferChar", &self.ucColorTransferChar).field("ucColorMatrixCoef", &self.ucColorMatrixCoef).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_COLORSPACEINFO_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_CREDENTIAL_FLAGS(pub i32);
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(1i32);
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(2i32);
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(4i32);
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(8i32);
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(16i32);
impl ::core::marker::Copy for WMT_CREDENTIAL_FLAGS {}
impl ::core::clone::Clone for WMT_CREDENTIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_CREDENTIAL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_DRMLA_TRUST(pub i32);
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(0i32);
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(1i32);
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(2i32);
impl ::core::marker::Copy for WMT_DRMLA_TRUST {}
impl ::core::clone::Clone for WMT_DRMLA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_DRMLA_TRUST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_DRMLA_TRUST {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_DRMLA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_DRMLA_TRUST").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl ::core::clone::Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        Self {
            packetHeaderBuffer: self.packetHeaderBuffer.clone(),
            cPayloads: self.cPayloads,
            pPayloadHeaderBuffers: self.pPayloadHeaderBuffers,
            cPayloadDataFragments: self.cPayloadDataFragments,
            pPayloadDataFragments: self.pPayloadDataFragments,
        }
    }
}
impl ::core::fmt::Debug for WMT_FILESINK_DATA_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_FILESINK_DATA_UNIT").field("packetHeaderBuffer", &self.packetHeaderBuffer).field("cPayloads", &self.cPayloads).field("pPayloadHeaderBuffers", &self.pPayloadHeaderBuffers).field("cPayloadDataFragments", &self.cPayloadDataFragments).field("pPayloadDataFragments", &self.pPayloadDataFragments).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_FILESINK_DATA_UNIT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::core::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
impl ::core::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_FILESINK_MODE(pub i32);
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(1i32);
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(2i32);
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = WMT_FILESINK_MODE(4i32);
impl ::core::marker::Copy for WMT_FILESINK_MODE {}
impl ::core::clone::Clone for WMT_FILESINK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_FILESINK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_FILESINK_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_FILESINK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_FILESINK_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_IMAGE_TYPE(pub i32);
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(0i32);
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(1i32);
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(2i32);
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for WMT_IMAGE_TYPE {}
impl ::core::clone::Clone for WMT_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_INDEXER_TYPE(pub i32);
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(0i32);
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(1i32);
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(2i32);
impl ::core::marker::Copy for WMT_INDEXER_TYPE {}
impl ::core::clone::Clone for WMT_INDEXER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEXER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_INDEXER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_INDEXER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEXER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_INDEX_TYPE(pub i32);
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(1i32);
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(2i32);
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(3i32);
impl ::core::marker::Copy for WMT_INDEX_TYPE {}
impl ::core::clone::Clone for WMT_INDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_INDEX_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_INDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_MUSICSPEECH_CLASS_MODE(pub i32);
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(0i32);
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(1i32);
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(2i32);
impl ::core::marker::Copy for WMT_MUSICSPEECH_CLASS_MODE {}
impl ::core::clone::Clone for WMT_MUSICSPEECH_CLASS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_MUSICSPEECH_CLASS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_MUSICSPEECH_CLASS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_MUSICSPEECH_CLASS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_MUSICSPEECH_CLASS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_NET_PROTOCOL(pub i32);
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = WMT_NET_PROTOCOL(0i32);
impl ::core::marker::Copy for WMT_NET_PROTOCOL {}
impl ::core::clone::Clone for WMT_NET_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_NET_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_NET_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_NET_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_NET_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_OFFSET_FORMAT(pub i32);
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(0i32);
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(1i32);
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(2i32);
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(3i32);
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(4i32);
impl ::core::marker::Copy for WMT_OFFSET_FORMAT {}
impl ::core::clone::Clone for WMT_OFFSET_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_OFFSET_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_OFFSET_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_OFFSET_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_OFFSET_FORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl ::core::clone::Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        Self { dwPayloadIndex: self.dwPayloadIndex, segmentData: self.segmentData.clone() }
    }
}
impl ::core::fmt::Debug for WMT_PAYLOAD_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_PAYLOAD_FRAGMENT").field("dwPayloadIndex", &self.dwPayloadIndex).field("segmentData", &self.segmentData).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_PAYLOAD_FRAGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::core::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
impl ::core::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_PLAY_MODE(pub i32);
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = WMT_PLAY_MODE(0i32);
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = WMT_PLAY_MODE(1i32);
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = WMT_PLAY_MODE(2i32);
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = WMT_PLAY_MODE(3i32);
impl ::core::marker::Copy for WMT_PLAY_MODE {}
impl ::core::clone::Clone for WMT_PLAY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PLAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_PLAY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_PLAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PLAY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_PROXY_SETTINGS(pub i32);
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(0i32);
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(1i32);
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(2i32);
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(3i32);
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(4i32);
impl ::core::marker::Copy for WMT_PROXY_SETTINGS {}
impl ::core::clone::Clone for WMT_PROXY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PROXY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_PROXY_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PROXY_SETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_RIGHTS(pub i32);
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = WMT_RIGHTS(1i32);
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(2i32);
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = WMT_RIGHTS(8i32);
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(16i32);
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = WMT_RIGHTS(32i32);
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = WMT_RIGHTS(64i32);
pub const WMT_RIGHT_COPY: WMT_RIGHTS = WMT_RIGHTS(128i32);
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = WMT_RIGHTS(256i32);
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = WMT_RIGHTS(65536i32);
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = WMT_RIGHTS(131072i32);
impl ::core::marker::Copy for WMT_RIGHTS {}
impl ::core::clone::Clone for WMT_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_RIGHTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_RIGHTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STATUS(pub i32);
pub const WMT_ERROR: WMT_STATUS = WMT_STATUS(0i32);
pub const WMT_OPENED: WMT_STATUS = WMT_STATUS(1i32);
pub const WMT_BUFFERING_START: WMT_STATUS = WMT_STATUS(2i32);
pub const WMT_BUFFERING_STOP: WMT_STATUS = WMT_STATUS(3i32);
pub const WMT_EOF: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_END_OF_FILE: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_END_OF_SEGMENT: WMT_STATUS = WMT_STATUS(5i32);
pub const WMT_END_OF_STREAMING: WMT_STATUS = WMT_STATUS(6i32);
pub const WMT_LOCATING: WMT_STATUS = WMT_STATUS(7i32);
pub const WMT_CONNECTING: WMT_STATUS = WMT_STATUS(8i32);
pub const WMT_NO_RIGHTS: WMT_STATUS = WMT_STATUS(9i32);
pub const WMT_MISSING_CODEC: WMT_STATUS = WMT_STATUS(10i32);
pub const WMT_STARTED: WMT_STATUS = WMT_STATUS(11i32);
pub const WMT_STOPPED: WMT_STATUS = WMT_STATUS(12i32);
pub const WMT_CLOSED: WMT_STATUS = WMT_STATUS(13i32);
pub const WMT_STRIDING: WMT_STATUS = WMT_STATUS(14i32);
pub const WMT_TIMER: WMT_STATUS = WMT_STATUS(15i32);
pub const WMT_INDEX_PROGRESS: WMT_STATUS = WMT_STATUS(16i32);
pub const WMT_SAVEAS_START: WMT_STATUS = WMT_STATUS(17i32);
pub const WMT_SAVEAS_STOP: WMT_STATUS = WMT_STATUS(18i32);
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = WMT_STATUS(19i32);
pub const WMT_NEW_METADATA: WMT_STATUS = WMT_STATUS(20i32);
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = WMT_STATUS(21i32);
pub const WMT_SOURCE_SWITCH: WMT_STATUS = WMT_STATUS(22i32);
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = WMT_STATUS(23i32);
pub const WMT_INDIVIDUALIZE: WMT_STATUS = WMT_STATUS(24i32);
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = WMT_STATUS(25i32);
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = WMT_STATUS(26i32);
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = WMT_STATUS(27i32);
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = WMT_STATUS(28i32);
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = WMT_STATUS(29i32);
pub const WMT_ERROR_WITHURL: WMT_STATUS = WMT_STATUS(30i32);
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = WMT_STATUS(31i32);
pub const WMT_CLIENT_CONNECT: WMT_STATUS = WMT_STATUS(32i32);
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = WMT_STATUS(33i32);
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = WMT_STATUS(34i32);
pub const WMT_RECONNECT_START: WMT_STATUS = WMT_STATUS(35i32);
pub const WMT_RECONNECT_END: WMT_STATUS = WMT_STATUS(36i32);
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = WMT_STATUS(37i32);
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = WMT_STATUS(38i32);
pub const WMT_SET_FEC_SPAN: WMT_STATUS = WMT_STATUS(39i32);
pub const WMT_PREROLL_READY: WMT_STATUS = WMT_STATUS(40i32);
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = WMT_STATUS(41i32);
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = WMT_STATUS(42i32);
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = WMT_STATUS(43i32);
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = WMT_STATUS(44i32);
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = WMT_STATUS(45i32);
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = WMT_STATUS(46i32);
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = WMT_STATUS(47i32);
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = WMT_STATUS(48i32);
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = WMT_STATUS(49i32);
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = WMT_STATUS(50i32);
pub const WMT_CONTENT_ENABLER: WMT_STATUS = WMT_STATUS(51i32);
impl ::core::marker::Copy for WMT_STATUS {}
impl ::core::clone::Clone for WMT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STORAGE_FORMAT(pub i32);
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(0i32);
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(1i32);
impl ::core::marker::Copy for WMT_STORAGE_FORMAT {}
impl ::core::clone::Clone for WMT_STORAGE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STORAGE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_STORAGE_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STORAGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STORAGE_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STREAM_SELECTION(pub i32);
pub const WMT_OFF: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(0i32);
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(1i32);
pub const WMT_ON: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(2i32);
impl ::core::marker::Copy for WMT_STREAM_SELECTION {}
impl ::core::clone::Clone for WMT_STREAM_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STREAM_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_STREAM_SELECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STREAM_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STREAM_SELECTION").field(&self.0).finish()
    }
}
#[repr(C, packed(2))]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl ::core::marker::Copy for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_TIMECODE_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WMT_TIMECODE_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_TIMECODE_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_TIMECODE_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_TIMECODE_FRAMERATE(pub i32);
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(0i32);
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(1i32);
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(2i32);
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(3i32);
impl ::core::marker::Copy for WMT_TIMECODE_FRAMERATE {}
impl ::core::clone::Clone for WMT_TIMECODE_FRAMERATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TIMECODE_FRAMERATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_TIMECODE_FRAMERATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_TIMECODE_FRAMERATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TIMECODE_FRAMERATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_TRANSPORT_TYPE(pub i32);
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(0i32);
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(1i32);
impl ::core::marker::Copy for WMT_TRANSPORT_TYPE {}
impl ::core::clone::Clone for WMT_TRANSPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_TRANSPORT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_VERSION(pub i32);
pub const WMT_VER_4_0: WMT_VERSION = WMT_VERSION(262144i32);
pub const WMT_VER_7_0: WMT_VERSION = WMT_VERSION(458752i32);
pub const WMT_VER_8_0: WMT_VERSION = WMT_VERSION(524288i32);
pub const WMT_VER_9_0: WMT_VERSION = WMT_VERSION(589824i32);
impl ::core::marker::Copy for WMT_VERSION {}
impl ::core::clone::Clone for WMT_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_VERSION").field(&self.0).finish()
    }
}
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[repr(C)]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE")
            .field("dwMagic", &self.dwMagic)
            .field("cbStruct", &self.cbStruct)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwInputFlagsCur", &self.dwInputFlagsCur)
            .field("lCurMotionXtoX", &self.lCurMotionXtoX)
            .field("lCurMotionYtoX", &self.lCurMotionYtoX)
            .field("lCurMotionXoffset", &self.lCurMotionXoffset)
            .field("lCurMotionXtoY", &self.lCurMotionXtoY)
            .field("lCurMotionYtoY", &self.lCurMotionYtoY)
            .field("lCurMotionYoffset", &self.lCurMotionYoffset)
            .field("lCurBlendCoef1", &self.lCurBlendCoef1)
            .field("lCurBlendCoef2", &self.lCurBlendCoef2)
            .field("dwInputFlagsPrev", &self.dwInputFlagsPrev)
            .field("lPrevMotionXtoX", &self.lPrevMotionXtoX)
            .field("lPrevMotionYtoX", &self.lPrevMotionYtoX)
            .field("lPrevMotionXoffset", &self.lPrevMotionXoffset)
            .field("lPrevMotionXtoY", &self.lPrevMotionXtoY)
            .field("lPrevMotionYtoY", &self.lPrevMotionYtoY)
            .field("lPrevMotionYoffset", &self.lPrevMotionYoffset)
            .field("lPrevBlendCoef1", &self.lPrevBlendCoef1)
            .field("lPrevBlendCoef2", &self.lPrevBlendCoef2)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_VIDEOIMAGE_SAMPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE2 {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE2")
            .field("dwMagic", &self.dwMagic)
            .field("dwStructSize", &self.dwStructSize)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwViewportWidth", &self.dwViewportWidth)
            .field("dwViewportHeight", &self.dwViewportHeight)
            .field("dwCurrImageWidth", &self.dwCurrImageWidth)
            .field("dwCurrImageHeight", &self.dwCurrImageHeight)
            .field("fCurrRegionX0", &self.fCurrRegionX0)
            .field("fCurrRegionY0", &self.fCurrRegionY0)
            .field("fCurrRegionWidth", &self.fCurrRegionWidth)
            .field("fCurrRegionHeight", &self.fCurrRegionHeight)
            .field("fCurrBlendCoef", &self.fCurrBlendCoef)
            .field("dwPrevImageWidth", &self.dwPrevImageWidth)
            .field("dwPrevImageHeight", &self.dwPrevImageHeight)
            .field("fPrevRegionX0", &self.fPrevRegionX0)
            .field("fPrevRegionY0", &self.fPrevRegionY0)
            .field("fPrevRegionWidth", &self.fPrevRegionWidth)
            .field("fPrevRegionHeight", &self.fPrevRegionHeight)
            .field("fPrevBlendCoef", &self.fPrevBlendCoef)
            .field("dwEffectType", &self.dwEffectType)
            .field("dwNumEffectParas", &self.dwNumEffectParas)
            .field("fEffectPara0", &self.fEffectPara0)
            .field("fEffectPara1", &self.fEffectPara1)
            .field("fEffectPara2", &self.fEffectPara2)
            .field("fEffectPara3", &self.fEffectPara3)
            .field("fEffectPara4", &self.fEffectPara4)
            .field("bKeepPrevImage", &self.bKeepPrevImage)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_VIDEOIMAGE_SAMPLE2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[repr(C)]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows_core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WATERMARK_ENTRY").field("wmetType", &self.wmetType).field("clsid", &self.clsid).field("cbDisplayName", &self.cbDisplayName).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_WATERMARK_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WATERMARK_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WATERMARK_ENTRY {}
impl ::core::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_WATERMARK_ENTRY_TYPE(pub i32);
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(1i32);
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(2i32);
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY_TYPE {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_WATERMARK_ENTRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WMT_WATERMARK_ENTRY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_WATERMARK_ENTRY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WMT_WEBSTREAM_FORMAT {}
impl ::core::clone::Clone for WMT_WEBSTREAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_FORMAT").field("cbSize", &self.cbSize).field("cbSampleHeaderFixedData", &self.cbSampleHeaderFixedData).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_WEBSTREAM_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
impl ::core::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl ::core::marker::Copy for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::clone::Clone for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_SAMPLE_HEADER").field("cbLength", &self.cbLength).field("wPart", &self.wPart).field("cTotalParts", &self.cTotalParts).field("wSampleType", &self.wSampleType).field("wszURL", &self.wszURL).finish()
    }
}
unsafe impl ::windows_core::Abi for WMT_WEBSTREAM_SAMPLE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_SAMPLE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: ::win32_foundation::RECT,
    pub rcTarget: ::win32_foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: ::win32_graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for WMVIDEOINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for WMVIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for WMVIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: ::win32_foundation::RECT,
    pub rcTarget: ::win32_foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: ::win32_graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for WMVIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for WMVIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER2")
            .field("rcSource", &self.rcSource)
            .field("rcTarget", &self.rcTarget)
            .field("dwBitRate", &self.dwBitRate)
            .field("dwBitErrorRate", &self.dwBitErrorRate)
            .field("AvgTimePerFrame", &self.AvgTimePerFrame)
            .field("dwInterlaceFlags", &self.dwInterlaceFlags)
            .field("dwCopyProtectFlags", &self.dwCopyProtectFlags)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("bmiHeader", &self.bmiHeader)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for WMVIDEOINFOHEADER2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WM_ADDRESS_ACCESSENTRY {}
impl ::core::clone::Clone for WM_ADDRESS_ACCESSENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_ADDRESS_ACCESSENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_ADDRESS_ACCESSENTRY").field("dwIPAddress", &self.dwIPAddress).field("dwMask", &self.dwMask).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_ADDRESS_ACCESSENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_ADDRESS_ACCESSENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
impl ::core::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_AETYPE(pub i32);
pub const WM_AETYPE_INCLUDE: WM_AETYPE = WM_AETYPE(105i32);
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = WM_AETYPE(101i32);
impl ::core::marker::Copy for WM_AETYPE {}
impl ::core::clone::Clone for WM_AETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_AETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_AETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_AETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_AETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES").field("dwIPAddress", &self.dwIPAddress).field("dwPort", &self.dwPort).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_CLIENT_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: ::windows_core::PCWSTR,
    pub pwszPort: ::windows_core::PCWSTR,
    pub pwszDNSName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES_EX {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES_EX").field("cbSize", &self.cbSize).field("pwszIPAddress", &self.pwszIPAddress).field("pwszPort", &self.pwszPort).field("pwszDNSName", &self.pwszDNSName).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_CLIENT_PROPERTIES_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_CL_INTERLACED420: u32 = 0u32;
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
pub const WM_CT_INTERLACED: u32 = 128u32;
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_DM_INTERLACED_TYPE(pub i32);
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(0i32);
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(1i32);
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(2i32);
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(3i32);
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(4i32);
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(5i32);
impl ::core::marker::Copy for WM_DM_INTERLACED_TYPE {}
impl ::core::clone::Clone for WM_DM_INTERLACED_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_INTERLACED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_DM_INTERLACED_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_DM_INTERLACED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_INTERLACED_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(pub i32);
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(0i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(1i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(2i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(3i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(4i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(5i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(6i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(7i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(8i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(9i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(10i32);
impl ::core::marker::Copy for WM_DM_IT_FIRST_FRAME_COHERENCY {}
impl ::core::clone::Clone for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_DM_IT_FIRST_FRAME_COHERENCY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_IT_FIRST_FRAME_COHERENCY").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for WM_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for WM_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_LEAKY_BUCKET_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_LEAKY_BUCKET_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_LEAKY_BUCKET_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_LEAKY_BUCKET_PAIR {}
impl ::core::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_MAX_STREAMS: u32 = 63u32;
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[repr(C)]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows_core::GUID,
    pub subtype: ::windows_core::GUID,
    pub bFixedSizeSamples: ::win32_foundation::BOOL,
    pub bTemporalCompression: ::win32_foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_core::GUID,
    pub pUnk: ::core::option::Option<::windows_core::IUnknown>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl ::core::clone::Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        Self {
            majortype: self.majortype,
            subtype: self.subtype,
            bFixedSizeSamples: self.bFixedSizeSamples,
            bTemporalCompression: self.bTemporalCompression,
            lSampleSize: self.lSampleSize,
            formattype: self.formattype,
            pUnk: self.pUnk.clone(),
            cbFormat: self.cbFormat,
            pbFormat: self.pbFormat,
        }
    }
}
impl ::core::fmt::Debug for WM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_MEDIA_TYPE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
impl ::core::cmp::Eq for WM_MEDIA_TYPE {}
impl ::core::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WM_PICTURE {
    pub pwszMIMEType: ::windows_core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: ::windows_core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WM_PICTURE {}
impl ::core::clone::Clone for WM_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_PICTURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_PICTURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PICTURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_PICTURE {}
impl ::core::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_PLAYBACK_DRC_LEVEL(pub i32);
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(0i32);
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(1i32);
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(2i32);
impl ::core::marker::Copy for WM_PLAYBACK_DRC_LEVEL {}
impl ::core::clone::Clone for WM_PLAYBACK_DRC_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_PLAYBACK_DRC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_PLAYBACK_DRC_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_PLAYBACK_DRC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_PLAYBACK_DRC_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl ::core::marker::Copy for WM_PORT_NUMBER_RANGE {}
impl ::core::clone::Clone for WM_PORT_NUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_PORT_NUMBER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_PORT_NUMBER_RANGE").field("wPortBegin", &self.wPortBegin).field("wPortEnd", &self.wPortEnd).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_PORT_NUMBER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PORT_NUMBER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_PORT_NUMBER_RANGE {}
impl ::core::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: ::windows_core::PWSTR,
    pub wszBrowserUserAgent: ::windows_core::PWSTR,
    pub wszBrowserWebPage: ::windows_core::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut ::win32_foundation::LPARAM,
    pub wszHostExe: ::windows_core::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WM_READER_CLIENTINFO {}
impl ::core::clone::Clone for WM_READER_CLIENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_READER_CLIENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_CLIENTINFO").field("cbSize", &self.cbSize).field("wszLang", &self.wszLang).field("wszBrowserUserAgent", &self.wszBrowserUserAgent).field("wszBrowserWebPage", &self.wszBrowserWebPage).field("qwReserved", &self.qwReserved).field("pReserved", &self.pReserved).field("wszHostExe", &self.wszHostExe).field("qwHostVersion", &self.qwHostVersion).field("wszPlayerUserAgent", &self.wszPlayerUserAgent).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_READER_CLIENTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_CLIENTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_READER_CLIENTINFO {}
impl ::core::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl ::core::marker::Copy for WM_READER_STATISTICS {}
impl ::core::clone::Clone for WM_READER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_READER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_STATISTICS").field("cbSize", &self.cbSize).field("dwBandwidth", &self.dwBandwidth).field("cPacketsReceived", &self.cPacketsReceived).field("cPacketsRecovered", &self.cPacketsRecovered).field("cPacketsLost", &self.cPacketsLost).field("wQuality", &self.wQuality).finish()
    }
}
unsafe impl ::windows_core::Abi for WM_READER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_READER_STATISTICS {}
impl ::core::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_SFEX_TYPE(pub i32);
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = WM_SFEX_TYPE(2i32);
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = WM_SFEX_TYPE(4i32);
impl ::core::marker::Copy for WM_SFEX_TYPE {}
impl ::core::clone::Clone for WM_SFEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SFEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_SFEX_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_SFEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SFEX_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_SF_TYPE(pub i32);
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = WM_SF_TYPE(1i32);
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = WM_SF_TYPE(2i32);
pub const WM_SF_DATALOSS: WM_SF_TYPE = WM_SF_TYPE(4i32);
impl ::core::marker::Copy for WM_SF_TYPE {}
impl ::core::clone::Clone for WM_SF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WM_SF_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SF_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(2))]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for WM_STREAM_PRIORITY_RECORD {}
impl ::core::clone::Clone for WM_STREAM_PRIORITY_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_STREAM_PRIORITY_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_STREAM_PRIORITY_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_PRIORITY_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_STREAM_PRIORITY_RECORD {}
impl ::core::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows_core::GUID,
    pub cbFormat: u32,
}
impl ::core::marker::Copy for WM_STREAM_TYPE_INFO {}
impl ::core::clone::Clone for WM_STREAM_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_STREAM_TYPE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_STREAM_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_TYPE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_STREAM_TYPE_INFO {}
impl ::core::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: ::windows_core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl ::core::marker::Copy for WM_SYNCHRONISED_LYRICS {}
impl ::core::clone::Clone for WM_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_SYNCHRONISED_LYRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_SYNCHRONISED_LYRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_SYNCHRONISED_LYRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_SYNCHRONISED_LYRICS {}
impl ::core::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[repr(C, packed(1))]
pub struct WM_USER_TEXT {
    pub pwszDescription: ::windows_core::PWSTR,
    pub pwszText: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_TEXT {}
impl ::core::clone::Clone for WM_USER_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_USER_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_USER_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_USER_TEXT {}
impl ::core::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: ::windows_core::PWSTR,
    pub pwszURL: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_WEB_URL {}
impl ::core::clone::Clone for WM_USER_WEB_URL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for WM_USER_WEB_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_USER_WEB_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_WEB_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_USER_WEB_URL {}
impl ::core::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS")
            .field("qwSampleCount", &self.qwSampleCount)
            .field("qwByteCount", &self.qwByteCount)
            .field("qwDroppedSampleCount", &self.qwDroppedSampleCount)
            .field("qwDroppedByteCount", &self.qwDroppedByteCount)
            .field("dwCurrentBitrate", &self.dwCurrentBitrate)
            .field("dwAverageBitrate", &self.dwAverageBitrate)
            .field("dwExpectedBitrate", &self.dwExpectedBitrate)
            .field("dwCurrentSampleRate", &self.dwCurrentSampleRate)
            .field("dwAverageSampleRate", &self.dwAverageSampleRate)
            .field("dwExpectedSampleRate", &self.dwExpectedSampleRate)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WM_WRITER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS {}
impl ::core::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS_EX {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS_EX")
            .field("dwBitratePlusOverhead", &self.dwBitratePlusOverhead)
            .field("dwCurrentSampleDropRateInQueue", &self.dwCurrentSampleDropRateInQueue)
            .field("dwCurrentSampleDropRateInCodec", &self.dwCurrentSampleDropRateInCodec)
            .field("dwCurrentSampleDropRateInMultiplexer", &self.dwCurrentSampleDropRateInMultiplexer)
            .field("dwTotalSampleDropsInQueue", &self.dwTotalSampleDropsInQueue)
            .field("dwTotalSampleDropsInCodec", &self.dwTotalSampleDropsInCodec)
            .field("dwTotalSampleDropsInMultiplexer", &self.dwTotalSampleDropsInMultiplexer)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for WM_WRITER_STATISTICS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS_EX {}
impl ::core::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _AM_ASFWRITERCONFIG_PARAM(pub i32);
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(1i32);
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(2i32);
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(3i32);
impl ::core::marker::Copy for _AM_ASFWRITERCONFIG_PARAM {}
impl ::core::clone::Clone for _AM_ASFWRITERCONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AM_ASFWRITERCONFIG_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _AM_ASFWRITERCONFIG_PARAM {
    type Abi = Self;
}
impl ::core::fmt::Debug for _AM_ASFWRITERCONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AM_ASFWRITERCONFIG_PARAM").field(&self.0).finish()
    }
}
pub const g_dwWMContentAttributes: u32 = 5u32;
pub const g_dwWMNSCAttributes: u32 = 5u32;
pub const g_dwWMSpecialAttributes: u32 = 20u32;
pub const g_wszASFLeakyBucketPairs: &str = "ASFLeakyBucketPairs";
pub const g_wszAllowInterlacedOutput: &str = "AllowInterlacedOutput";
pub const g_wszAverageLevel: &str = "AverageLevel";
pub const g_wszBufferAverage: &str = "Buffer Average";
pub const g_wszComplexity: &str = "_COMPLEXITYEX";
pub const g_wszComplexityLive: &str = "_COMPLEXITYEXLIVE";
pub const g_wszComplexityMax: &str = "_COMPLEXITYEXMAX";
pub const g_wszComplexityOffline: &str = "_COMPLEXITYEXOFFLINE";
pub const g_wszDecoderComplexityRequested: &str = "_DECODERCOMPLEXITYPROFILE";
pub const g_wszDedicatedDeliveryThread: &str = "DedicatedDeliveryThread";
pub const g_wszDeinterlaceMode: &str = "DeinterlaceMode";
pub const g_wszDeliverOnReceive: &str = "DeliverOnReceive";
pub const g_wszDeviceConformanceTemplate: &str = "DeviceConformanceTemplate";
pub const g_wszDynamicRangeControl: &str = "DynamicRangeControl";
pub const g_wszEDL: &str = "_EDL";
pub const g_wszEarlyDataDelivery: &str = "EarlyDataDelivery";
pub const g_wszEnableDiscreteOutput: &str = "EnableDiscreteOutput";
pub const g_wszEnableFrameInterpolation: &str = "EnableFrameInterpolation";
pub const g_wszEnableWMAProSPDIFOutput: &str = "EnableWMAProSPDIFOutput";
pub const g_wszFailSeekOnError: &str = "FailSeekOnError";
pub const g_wszFixedFrameRate: &str = "FixedFrameRate";
pub const g_wszFold6To2Channels3: &str = "Fold6To2Channels3";
pub const g_wszFoldToChannelsTemplate: &str = "Fold%luTo%luChannels%lu";
pub const g_wszInitialPatternForInverseTelecine: &str = "InitialPatternForInverseTelecine";
pub const g_wszInterlacedCoding: &str = "InterlacedCoding";
pub const g_wszIsVBRSupported: &str = "_ISVBRSUPPORTED";
pub const g_wszJPEGCompressionQuality: &str = "JPEGCompressionQuality";
pub const g_wszJustInTimeDecode: &str = "JustInTimeDecode";
pub const g_wszMixedClassMode: &str = "MixedClassMode";
pub const g_wszMusicClassMode: &str = "MusicClassMode";
pub const g_wszMusicSpeechClassMode: &str = "MusicSpeechClassMode";
pub const g_wszNeedsPreviousSample: &str = "NeedsPreviousSample";
pub const g_wszNumPasses: &str = "_PASSESUSED";
pub const g_wszOriginalSourceFormatTag: &str = "_SOURCEFORMATTAG";
pub const g_wszOriginalWaveFormat: &str = "_ORIGINALWAVEFORMAT";
pub const g_wszPeakValue: &str = "PeakValue";
pub const g_wszPermitSeeksBeyondEndOfStream: &str = "PermitSeeksBeyondEndOfStream";
pub const g_wszReloadIndexOnSeek: &str = "ReloadIndexOnSeek";
pub const g_wszScrambledAudio: &str = "ScrambledAudio";
pub const g_wszSingleOutputBuffer: &str = "SingleOutputBuffer";
pub const g_wszSoftwareScaling: &str = "SoftwareScaling";
pub const g_wszSourceBufferTime: &str = "SourceBufferTime";
pub const g_wszSourceMaxBytesAtOnce: &str = "SourceMaxBytesAtOnce";
pub const g_wszSpeakerConfig: &str = "SpeakerConfig";
pub const g_wszSpeechCaps: &str = "SpeechFormatCap";
pub const g_wszSpeechClassMode: &str = "SpeechClassMode";
pub const g_wszStreamLanguage: &str = "StreamLanguage";
pub const g_wszStreamNumIndexObjects: &str = "StreamNumIndexObjects";
pub const g_wszUsePacketAtSeekPoint: &str = "UsePacketAtSeekPoint";
pub const g_wszVBRBitrateMax: &str = "_RMAX";
pub const g_wszVBRBufferWindowMax: &str = "_BMAX";
pub const g_wszVBREnabled: &str = "_VBRENABLED";
pub const g_wszVBRPeak: &str = "VBR Peak";
pub const g_wszVBRQuality: &str = "_VBRQUALITY";
pub const g_wszVideoSampleDurations: &str = "VideoSampleDurations";
pub const g_wszWMADID: &str = "WM/ADID";
pub const g_wszWMASFPacketCount: &str = "WM/ASFPacketCount";
pub const g_wszWMASFSecurityObjectsSize: &str = "WM/ASFSecurityObjectsSize";
pub const g_wszWMAlbumArtist: &str = "WM/AlbumArtist";
pub const g_wszWMAlbumArtistSort: &str = "WM/AlbumArtistSort";
pub const g_wszWMAlbumCoverURL: &str = "WM/AlbumCoverURL";
pub const g_wszWMAlbumTitle: &str = "WM/AlbumTitle";
pub const g_wszWMAlbumTitleSort: &str = "WM/AlbumTitleSort";
pub const g_wszWMAspectRatioX: &str = "AspectRatioX";
pub const g_wszWMAspectRatioY: &str = "AspectRatioY";
pub const g_wszWMAudioFileURL: &str = "WM/AudioFileURL";
pub const g_wszWMAudioSourceURL: &str = "WM/AudioSourceURL";
pub const g_wszWMAuthor: &str = "Author";
pub const g_wszWMAuthorSort: &str = "AuthorSort";
pub const g_wszWMAuthorURL: &str = "WM/AuthorURL";
pub const g_wszWMBannerImageData: &str = "BannerImageData";
pub const g_wszWMBannerImageType: &str = "BannerImageType";
pub const g_wszWMBannerImageURL: &str = "BannerImageURL";
pub const g_wszWMBeatsPerMinute: &str = "WM/BeatsPerMinute";
pub const g_wszWMBitrate: &str = "Bitrate";
pub const g_wszWMBroadcast: &str = "Broadcast";
pub const g_wszWMCategory: &str = "WM/Category";
pub const g_wszWMCodec: &str = "WM/Codec";
pub const g_wszWMComposer: &str = "WM/Composer";
pub const g_wszWMComposerSort: &str = "WM/ComposerSort";
pub const g_wszWMConductor: &str = "WM/Conductor";
pub const g_wszWMContainerFormat: &str = "WM/ContainerFormat";
pub const g_wszWMContentDistributor: &str = "WM/ContentDistributor";
pub const g_wszWMContentGroupDescription: &str = "WM/ContentGroupDescription";
pub const g_wszWMCopyright: &str = "Copyright";
pub const g_wszWMCopyrightURL: &str = "CopyrightURL";
pub const g_wszWMCurrentBitrate: &str = "CurrentBitrate";
pub const g_wszWMDRM: &str = "WM/DRM";
pub const g_wszWMDRM_ContentID: &str = "DRM_ContentID";
pub const g_wszWMDRM_Flags: &str = "DRM_Flags";
pub const g_wszWMDRM_HeaderSignPrivKey: &str = "DRM_HeaderSignPrivKey";
pub const g_wszWMDRM_IndividualizedVersion: &str = "DRM_IndividualizedVersion";
pub const g_wszWMDRM_KeyID: &str = "DRM_KeyID";
pub const g_wszWMDRM_KeySeed: &str = "DRM_KeySeed";
pub const g_wszWMDRM_LASignatureCert: &str = "DRM_LASignatureCert";
pub const g_wszWMDRM_LASignatureLicSrvCert: &str = "DRM_LASignatureLicSrvCert";
pub const g_wszWMDRM_LASignaturePrivKey: &str = "DRM_LASignaturePrivKey";
pub const g_wszWMDRM_LASignatureRootCert: &str = "DRM_LASignatureRootCert";
pub const g_wszWMDRM_Level: &str = "DRM_Level";
pub const g_wszWMDRM_LicenseAcqURL: &str = "DRM_LicenseAcqURL";
pub const g_wszWMDRM_SourceID: &str = "DRM_SourceID";
pub const g_wszWMDRM_V1LicenseAcqURL: &str = "DRM_V1LicenseAcqURL";
pub const g_wszWMDVDID: &str = "WM/DVDID";
pub const g_wszWMDescription: &str = "Description";
pub const g_wszWMDirector: &str = "WM/Director";
pub const g_wszWMDuration: &str = "Duration";
pub const g_wszWMEncodedBy: &str = "WM/EncodedBy";
pub const g_wszWMEncodingSettings: &str = "WM/EncodingSettings";
pub const g_wszWMEncodingTime: &str = "WM/EncodingTime";
pub const g_wszWMEpisodeNumber: &str = "WM/EpisodeNumber";
pub const g_wszWMFileSize: &str = "FileSize";
pub const g_wszWMGenre: &str = "WM/Genre";
pub const g_wszWMGenreID: &str = "WM/GenreID";
pub const g_wszWMHasArbitraryDataStream: &str = "HasArbitraryDataStream";
pub const g_wszWMHasAttachedImages: &str = "HasAttachedImages";
pub const g_wszWMHasAudio: &str = "HasAudio";
pub const g_wszWMHasFileTransferStream: &str = "HasFileTransferStream";
pub const g_wszWMHasImage: &str = "HasImage";
pub const g_wszWMHasScript: &str = "HasScript";
pub const g_wszWMHasVideo: &str = "HasVideo";
pub const g_wszWMISAN: &str = "WM/ISAN";
pub const g_wszWMISRC: &str = "WM/ISRC";
pub const g_wszWMInitialKey: &str = "WM/InitialKey";
pub const g_wszWMIsCompilation: &str = "WM/IsCompilation";
pub const g_wszWMIsVBR: &str = "IsVBR";
pub const g_wszWMLanguage: &str = "WM/Language";
pub const g_wszWMLyrics: &str = "WM/Lyrics";
pub const g_wszWMLyrics_Synchronised: &str = "WM/Lyrics_Synchronised";
pub const g_wszWMMCDI: &str = "WM/MCDI";
pub const g_wszWMMediaClassPrimaryID: &str = "WM/MediaClassPrimaryID";
pub const g_wszWMMediaClassSecondaryID: &str = "WM/MediaClassSecondaryID";
pub const g_wszWMMediaCredits: &str = "WM/MediaCredits";
pub const g_wszWMMediaIsDelay: &str = "WM/MediaIsDelay";
pub const g_wszWMMediaIsFinale: &str = "WM/MediaIsFinale";
pub const g_wszWMMediaIsLive: &str = "WM/MediaIsLive";
pub const g_wszWMMediaIsPremiere: &str = "WM/MediaIsPremiere";
pub const g_wszWMMediaIsRepeat: &str = "WM/MediaIsRepeat";
pub const g_wszWMMediaIsSAP: &str = "WM/MediaIsSAP";
pub const g_wszWMMediaIsStereo: &str = "WM/MediaIsStereo";
pub const g_wszWMMediaIsSubtitled: &str = "WM/MediaIsSubtitled";
pub const g_wszWMMediaIsTape: &str = "WM/MediaIsTape";
pub const g_wszWMMediaNetworkAffiliation: &str = "WM/MediaNetworkAffiliation";
pub const g_wszWMMediaOriginalBroadcastDateTime: &str = "WM/MediaOriginalBroadcastDateTime";
pub const g_wszWMMediaOriginalChannel: &str = "WM/MediaOriginalChannel";
pub const g_wszWMMediaStationCallSign: &str = "WM/MediaStationCallSign";
pub const g_wszWMMediaStationName: &str = "WM/MediaStationName";
pub const g_wszWMModifiedBy: &str = "WM/ModifiedBy";
pub const g_wszWMMood: &str = "WM/Mood";
pub const g_wszWMNSCAddress: &str = "NSC_Address";
pub const g_wszWMNSCDescription: &str = "NSC_Description";
pub const g_wszWMNSCEmail: &str = "NSC_Email";
pub const g_wszWMNSCName: &str = "NSC_Name";
pub const g_wszWMNSCPhone: &str = "NSC_Phone";
pub const g_wszWMNumberOfFrames: &str = "NumberOfFrames";
pub const g_wszWMOptimalBitrate: &str = "OptimalBitrate";
pub const g_wszWMOriginalAlbumTitle: &str = "WM/OriginalAlbumTitle";
pub const g_wszWMOriginalArtist: &str = "WM/OriginalArtist";
pub const g_wszWMOriginalFilename: &str = "WM/OriginalFilename";
pub const g_wszWMOriginalLyricist: &str = "WM/OriginalLyricist";
pub const g_wszWMOriginalReleaseTime: &str = "WM/OriginalReleaseTime";
pub const g_wszWMOriginalReleaseYear: &str = "WM/OriginalReleaseYear";
pub const g_wszWMParentalRating: &str = "WM/ParentalRating";
pub const g_wszWMParentalRatingReason: &str = "WM/ParentalRatingReason";
pub const g_wszWMPartOfSet: &str = "WM/PartOfSet";
pub const g_wszWMPeakBitrate: &str = "WM/PeakBitrate";
pub const g_wszWMPeriod: &str = "WM/Period";
pub const g_wszWMPicture: &str = "WM/Picture";
pub const g_wszWMPlaylistDelay: &str = "WM/PlaylistDelay";
pub const g_wszWMProducer: &str = "WM/Producer";
pub const g_wszWMPromotionURL: &str = "WM/PromotionURL";
pub const g_wszWMProtected: &str = "Is_Protected";
pub const g_wszWMProtectionType: &str = "WM/ProtectionType";
pub const g_wszWMProvider: &str = "WM/Provider";
pub const g_wszWMProviderCopyright: &str = "WM/ProviderCopyright";
pub const g_wszWMProviderRating: &str = "WM/ProviderRating";
pub const g_wszWMProviderStyle: &str = "WM/ProviderStyle";
pub const g_wszWMPublisher: &str = "WM/Publisher";
pub const g_wszWMRadioStationName: &str = "WM/RadioStationName";
pub const g_wszWMRadioStationOwner: &str = "WM/RadioStationOwner";
pub const g_wszWMRating: &str = "Rating";
pub const g_wszWMSeasonNumber: &str = "WM/SeasonNumber";
pub const g_wszWMSeekable: &str = "Seekable";
pub const g_wszWMSharedUserRating: &str = "WM/SharedUserRating";
pub const g_wszWMSignature_Name: &str = "Signature_Name";
pub const g_wszWMSkipBackward: &str = "Can_Skip_Backward";
pub const g_wszWMSkipForward: &str = "Can_Skip_Forward";
pub const g_wszWMStreamTypeInfo: &str = "WM/StreamTypeInfo";
pub const g_wszWMStridable: &str = "Stridable";
pub const g_wszWMSubTitle: &str = "WM/SubTitle";
pub const g_wszWMSubTitleDescription: &str = "WM/SubTitleDescription";
pub const g_wszWMSubscriptionContentID: &str = "WM/SubscriptionContentID";
pub const g_wszWMText: &str = "WM/Text";
pub const g_wszWMTitle: &str = "Title";
pub const g_wszWMTitleSort: &str = "TitleSort";
pub const g_wszWMToolName: &str = "WM/ToolName";
pub const g_wszWMToolVersion: &str = "WM/ToolVersion";
pub const g_wszWMTrack: &str = "WM/Track";
pub const g_wszWMTrackNumber: &str = "WM/TrackNumber";
pub const g_wszWMTrusted: &str = "Is_Trusted";
pub const g_wszWMUniqueFileIdentifier: &str = "WM/UniqueFileIdentifier";
pub const g_wszWMUse_Advanced_DRM: &str = "Use_Advanced_DRM";
pub const g_wszWMUse_DRM: &str = "Use_DRM";
pub const g_wszWMUserWebURL: &str = "WM/UserWebURL";
pub const g_wszWMVideoClosedCaptioning: &str = "WM/VideoClosedCaptioning";
pub const g_wszWMVideoFrameRate: &str = "WM/VideoFrameRate";
pub const g_wszWMVideoHeight: &str = "WM/VideoHeight";
pub const g_wszWMVideoWidth: &str = "WM/VideoWidth";
pub const g_wszWMWMADRCAverageReference: &str = "WM/WMADRCAverageReference";
pub const g_wszWMWMADRCAverageTarget: &str = "WM/WMADRCAverageTarget";
pub const g_wszWMWMADRCPeakReference: &str = "WM/WMADRCPeakReference";
pub const g_wszWMWMADRCPeakTarget: &str = "WM/WMADRCPeakTarget";
pub const g_wszWMWMCPDistributor: &str = "WM/WMCPDistributor";
pub const g_wszWMWMCPDistributorID: &str = "WM/WMCPDistributorID";
pub const g_wszWMWMCollectionGroupID: &str = "WM/WMCollectionGroupID";
pub const g_wszWMWMCollectionID: &str = "WM/WMCollectionID";
pub const g_wszWMWMContentID: &str = "WM/WMContentID";
pub const g_wszWMWMShadowFileSourceDRMType: &str = "WM/WMShadowFileSourceDRMType";
pub const g_wszWMWMShadowFileSourceFileType: &str = "WM/WMShadowFileSourceFileType";
pub const g_wszWMWriter: &str = "WM/Writer";
pub const g_wszWMYear: &str = "WM/Year";
pub const g_wszWatermarkCLSID: &str = "WatermarkCLSID";
pub const g_wszWatermarkConfig: &str = "WatermarkConfig";
