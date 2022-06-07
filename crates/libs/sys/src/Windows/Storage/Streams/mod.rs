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
impl ::windows_sys::core::Interface for IBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421821408, data2: 48211, data3: 4575, data4: [140, 73, 0, 30, 79, 198, 134, 218] };
}
#[repr(C)]
pub struct IBufferFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBufferFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1907331405, data2: 49423, data3: 18507, data4: [188, 80, 20, 188, 98, 59, 58, 39] };
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
impl ::windows_sys::core::Interface for IBufferStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3909215835, data2: 55062, data3: 18266, data4: [169, 10, 175, 114, 41, 177, 231, 65] };
}
#[repr(C)]
pub struct IContentTypeProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContentTypeProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547030181, data2: 15257, data3: 19945, data4: [136, 165, 225, 29, 47, 80, 199, 149] };
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
impl ::windows_sys::core::Interface for IDataReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803512873, data2: 46273, data3: 17172, data4: [164, 184, 251, 129, 58, 47, 39, 94] };
}
#[repr(C)]
pub struct IDataReaderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDataReader: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataReaderFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3612506183, data2: 22490, data3: 19989, data4: [145, 76, 6, 128, 102, 153, 160, 152] };
}
#[repr(C)]
pub struct IDataReaderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataReaderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 301776840, data2: 63802, data3: 18203, data4: [177, 33, 243, 121, 227, 73, 49, 60] };
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
impl ::windows_sys::core::Interface for IDataWriter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1689817701, data2: 54081, data3: 18722, data4: [179, 138, 221, 74, 248, 128, 140, 78] };
}
#[repr(C)]
pub struct IDataWriterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDataWriter: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDataWriterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 864839618, data2: 35716, data3: 19499, data4: [156, 80, 123, 135, 103, 132, 122, 31] };
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
impl ::windows_sys::core::Interface for IFileRandomAccessStreamStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1934950663, data2: 15191, data3: 19293, data4: [131, 69, 85, 77, 47, 198, 33, 240] };
}
#[repr(C)]
pub struct IInputStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadAsync: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsync: usize,
}
impl ::windows_sys::core::Interface for IInputStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421821410, data2: 48211, data3: 4575, data4: [140, 73, 0, 30, 79, 198, 134, 218] };
}
#[repr(C)]
pub struct IInputStreamReference {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenSequentialReadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenSequentialReadAsync: usize,
}
impl ::windows_sys::core::Interface for IInputStreamReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1133681944, data2: 24265, data3: 19290, data4: [145, 156, 66, 5, 176, 200, 4, 182] };
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
impl ::windows_sys::core::Interface for IOutputStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421821414, data2: 48211, data3: 4575, data4: [140, 73, 0, 30, 79, 198, 134, 218] };
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
impl ::windows_sys::core::Interface for IPropertySetSerializer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1854848796, data2: 61245, data3: 17270, data4: [178, 14, 91, 230, 56, 174, 172, 119] };
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
impl ::windows_sys::core::Interface for IRandomAccessStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421821409, data2: 48211, data3: 4575, data4: [140, 73, 0, 30, 79, 198, 134, 218] };
}
#[repr(C)]
pub struct IRandomAccessStreamReference {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenReadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenReadAsync: usize,
}
impl ::windows_sys::core::Interface for IRandomAccessStreamReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 871248180, data2: 7638, data3: 20026, data4: [128, 103, 209, 193, 98, 232, 100, 43] };
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
impl ::windows_sys::core::Interface for IRandomAccessStreamReferenceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238908892, data2: 16319, data3: 20093, data4: [152, 111, 239, 59, 26, 7, 169, 100] };
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
impl ::windows_sys::core::Interface for IRandomAccessStreamStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1380773327, data2: 28201, data3: 19685, data4: [149, 115, 107, 117, 61, 182, 108, 58] };
}
#[repr(C)]
pub struct IRandomAccessStreamWithContentType {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IRandomAccessStreamWithContentType {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3424995367, data2: 19261, data3: 17295, data4: [146, 50, 16, 199, 107, 199, 224, 56] };
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
