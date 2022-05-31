#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylist(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaylist {
    type Vtable = IPlaylist_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x803736f5_cf44_4d97_83b3_7a089e9ab663);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylist_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-storage")))]
    Files: usize,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub SaveAsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: ::windows_core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, option: ::winrt_storage::NameCollisionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SaveAsAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub SaveAsWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savelocation: ::windows_core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, option: ::winrt_storage::NameCollisionOption, playlistformat: PlaylistFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SaveAsWithFormatAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaylistStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaylistStatics {
    type Vtable = IPlaylistStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5c331cd_81f9_4ff3_95b9_70b6ff046b68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaylistStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    LoadAsync: usize,
}
#[repr(transparent)]
pub struct Playlist(::windows_core::IUnknown);
impl Playlist {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Playlist, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-storage"))]
    pub fn Files(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_storage::StorageFile>>(result__)
        }
    }
    pub fn SaveAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SaveAsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, savelocation: Param0, desiredname: Param1, option: ::winrt_storage::NameCollisionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsAsync)(::windows_core::Interface::as_raw(this), savelocation.into_param().abi(), desiredname.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SaveAsWithFormatAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, savelocation: Param0, desiredname: Param1, option: ::winrt_storage::NameCollisionOption, playlistformat: PlaylistFormat) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsWithFormatAsync)(::windows_core::Interface::as_raw(this), savelocation.into_param().abi(), desiredname.into_param().abi(), option, playlistformat, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::StorageFile>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn LoadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(file: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Playlist>> {
        Self::IPlaylistStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Playlist>>(result__)
        })
    }
    pub fn IPlaylistStatics<R, F: FnOnce(&IPlaylistStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Playlist, IPlaylistStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Playlist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Playlist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Playlist {}
impl ::core::fmt::Debug for Playlist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Playlist").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Playlist {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.Playlists.Playlist;{803736f5-cf44-4d97-83b3-7a089e9ab663})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Playlist {
    type Vtable = IPlaylist_Vtbl;
    const IID: ::windows_core::GUID = <IPlaylist as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Playlist {
    const NAME: &'static str = "Windows.Media.Playlists.Playlist";
}
impl ::core::convert::From<Playlist> for ::windows_core::IUnknown {
    fn from(value: Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Playlist> for ::windows_core::IUnknown {
    fn from(value: &Playlist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Playlist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Playlist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Playlist> for ::windows_core::IInspectable {
    fn from(value: Playlist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Playlist> for ::windows_core::IInspectable {
    fn from(value: &Playlist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Playlist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Playlist {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlaylistFormat(pub i32);
impl PlaylistFormat {
    pub const WindowsMedia: Self = Self(0i32);
    pub const Zune: Self = Self(1i32);
    pub const M3u: Self = Self(2i32);
}
impl ::core::marker::Copy for PlaylistFormat {}
impl ::core::clone::Clone for PlaylistFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlaylistFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlaylistFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlaylistFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaylistFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlaylistFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.Playlists.PlaylistFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
