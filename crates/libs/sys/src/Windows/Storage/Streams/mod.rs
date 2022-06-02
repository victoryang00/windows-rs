pub type Buffer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
}
impl ::core::marker::Copy for ByteOrder {}
impl ::core::clone::Clone for ByteOrder {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DataReader = *mut ::core::ffi::c_void;
pub type DataReaderLoadOperation = *mut ::core::ffi::c_void;
pub type DataWriter = *mut ::core::ffi::c_void;
pub type DataWriterStoreOperation = *mut ::core::ffi::c_void;
pub type FileInputStream = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
}
impl ::core::marker::Copy for FileOpenDisposition {}
impl ::core::clone::Clone for FileOpenDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FileOutputStream = *mut ::core::ffi::c_void;
pub type FileRandomAccessStream = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBufferFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBufferStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateCopyFromMemoryBuffer: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCopyFromMemoryBuffer: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMemoryBufferOverIBuffer: unsafe extern "system" fn(this: *mut *mut Self, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMemoryBufferOverIBuffer: usize,
}
#[repr(C)]
pub struct IContentTypeProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnconsumedBufferLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnicodeEncoding) -> ::windows_sys::core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut *mut Self, value: UnicodeEncoding) -> ::windows_sys::core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ByteOrder) -> ::windows_sys::core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ByteOrder) -> ::windows_sys::core::HRESULT,
    pub InputStreamOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InputStreamOptions) -> ::windows_sys::core::HRESULT,
    pub SetInputStreamOptions: unsafe extern "system" fn(this: *mut *mut Self, value: InputStreamOptions) -> ::windows_sys::core::HRESULT,
    pub ReadByte: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *mut u8) -> ::windows_sys::core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(this: *mut *mut Self, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReadBoolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ReadInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ReadInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReadInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub ReadUInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub ReadUInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReadUInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ReadSingle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub ReadDouble: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ReadString: unsafe extern "system" fn(this: *mut *mut Self, codeunitcount: u32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadDateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTimeSpan: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataReaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDataReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataReaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataWriter {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnstoredBufferLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnicodeEncoding) -> ::windows_sys::core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut *mut Self, value: UnicodeEncoding) -> ::windows_sys::core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ByteOrder) -> ::windows_sys::core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ByteOrder) -> ::windows_sys::core::HRESULT,
    pub WriteByte: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub WriteBytes: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, start: u32, count: u32) -> ::windows_sys::core::HRESULT,
    pub WriteBoolean: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(this: *mut *mut Self, value: i64) -> ::windows_sys::core::HRESULT,
    pub WriteUInt16: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub WriteUInt32: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub WriteUInt64: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub WriteSingle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub WriteDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WriteDateTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTimeSpan: usize,
    pub WriteString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataWriterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDataWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFileRandomAccessStreamStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, filepath: ::windows_sys::core::HSTRING, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, filepath: ::windows_sys::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, filepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, filepath: ::windows_sys::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserWithOptionsAsync: usize,
}
#[repr(C)]
pub struct IInputStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadAsync: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsync: usize,
}
#[repr(C)]
pub struct IInputStreamReference {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenSequentialReadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenSequentialReadAsync: usize,
}
#[repr(C)]
pub struct IOutputStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub WriteAsync: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
#[repr(C)]
pub struct IPropertySetSerializer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Serialize: unsafe extern "system" fn(this: *mut *mut Self, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Serialize: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Deserialize: unsafe extern "system" fn(this: *mut *mut Self, propertyset: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Deserialize: usize,
}
#[repr(C)]
pub struct IRandomAccessStream {
    pub base__: ::windows_sys::core::IInspectable,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(this: *mut *mut Self, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(this: *mut *mut Self, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, position: u64) -> ::windows_sys::core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRandomAccessStreamReference {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenReadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenReadAsync: usize,
}
#[repr(C)]
pub struct IRandomAccessStreamReferenceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromFile: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
    pub CreateFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRandomAccessStreamStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CopyAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopySizeAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, bytestocopy: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopySizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopyAndCloseAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAndCloseAsync: usize,
}
#[repr(C)]
pub struct IRandomAccessStreamWithContentType {
    pub base__: ::windows_sys::core::IInspectable,
}
pub type InMemoryRandomAccessStream = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
}
impl ::core::marker::Copy for InputStreamOptions {}
impl ::core::clone::Clone for InputStreamOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InputStreamOverStream = *mut ::core::ffi::c_void;
pub type OutputStreamOverStream = *mut ::core::ffi::c_void;
pub type RandomAccessStreamOverStream = *mut ::core::ffi::c_void;
pub type RandomAccessStreamReference = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for UnicodeEncoding {}
impl ::core::clone::Clone for UnicodeEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
