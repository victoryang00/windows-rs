#[repr(C)]
pub struct IPlaylist {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsAsync: unsafe extern "system" fn(this: *mut *mut Self, savelocation: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, option: super::super::Storage::NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsWithFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, savelocation: *mut ::core::ffi::c_void, desiredname: ::windows_sys::core::HSTRING, option: super::super::Storage::NameCollisionOption, playlistformat: PlaylistFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsWithFormatAsync: usize,
}
#[repr(C)]
pub struct IPlaylistStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadAsync: usize,
}
pub type Playlist = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playlists\"`*"]
#[repr(transparent)]
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
