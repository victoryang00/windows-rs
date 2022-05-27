#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformation(::windows_core::IUnknown);
impl FileInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &::windows_core::Interface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenSequentialReadAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows_core::Interface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenReadAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FileType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FileType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), accessmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyOverload)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyAndReplaceAsync)(::windows_core::Interface::as_raw(this), filetoreplace.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFolder>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveOverload)(::windows_core::Interface::as_raw(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageFile>>(&self, filetoreplace: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MoveAndReplaceAsync)(::windows_core::Interface::as_raw(this), filetoreplace.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenWithOptionsAsync)(::windows_core::Interface::as_raw(this), accessmode, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::windows_core::Interface::as_raw(this), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsAvailable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenameAsync)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Attributes(&self) -> ::windows_core::Result<super::FileAttributes> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::FileAttributes>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DateCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOfType)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetParentAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEqual)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BasicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::windows_core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), mode, requestedsize, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), mode, requestedsize, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FolderRelativeId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Provider(&self) -> ::windows_core::Result<super::StorageProvider> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Provider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformation {}
impl ::core::fmt::Debug for FileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows_core::GUID = <IStorageItemInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
impl ::core::convert::From<FileInformation> for ::windows_core::IUnknown {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows_core::IUnknown {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileInformation> for ::windows_core::IInspectable {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows_core::IInspectable {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, super::Streams::IInputStreamReference> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Streams::IInputStreamReference> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, super::Streams::IInputStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Streams::IInputStreamReference> {
        ::core::convert::TryInto::<super::Streams::IInputStreamReference>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, super::Streams::IRandomAccessStreamReference> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<super::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFile> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFile> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFile> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFile> {
        ::core::convert::TryInto::<super::IStorageFile>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFile2> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFile2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFile2> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFile2> {
        ::core::convert::TryInto::<super::IStorageFile2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFilePropertiesWithAvailability> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFilePropertiesWithAvailability> {
        ::core::convert::TryInto::<super::IStorageFilePropertiesWithAvailability>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem> {
        ::core::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem2> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem2> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem2> {
        ::core::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for IStorageItemInformation {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for IStorageItemInformation {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemInformation> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemInformation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemInformation> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemInformation> {
        ::core::convert::TryInto::<IStorageItemInformation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemProperties {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemProperties {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemProperties> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemProperties> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemProperties> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemProperties> {
        ::core::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: FileInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FileInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformationFactory(::windows_core::IUnknown);
impl FileInformationFactory {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedItemsVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetVirtualizedItemsVector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedFilesVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetVirtualizedFilesVector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn GetVirtualizedFoldersVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetVirtualizedFoldersVector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithMode<'a, Param0: ::windows_core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMode)(::windows_core::Interface::as_raw(this), queryresult.into_param().abi(), mode, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSize<'a, Param0: ::windows_core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows_core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithModeAndSize)(::windows_core::Interface::as_raw(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptions<'a, Param0: ::windows_core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithModeAndSizeAndOptions)(::windows_core::Interface::as_raw(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<'a, Param0: ::windows_core::IntoParam<'a, super::Search::IStorageQueryResultBase>>(queryresult: Param0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows_core::Result<FileInformationFactory> {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithModeAndSizeAndOptionsAndFlags)(::windows_core::Interface::as_raw(this), queryresult.into_param().abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FileInformationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformationFactory {}
impl ::core::fmt::Debug for FileInformationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileInformationFactory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformationFactory;{401d88be-960f-4d6d-a7d0-1a3861e76c83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows_core::GUID = <IFileInformationFactory as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
impl ::core::convert::From<FileInformationFactory> for ::windows_core::IUnknown {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows_core::IUnknown {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileInformationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileInformationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileInformationFactory> for ::windows_core::IInspectable {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows_core::IInspectable {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileInformationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileInformationFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FileInformationFactory {}
unsafe impl ::core::marker::Sync for FileInformationFactory {}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FolderInformation(::windows_core::IUnknown);
impl FolderInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileAsync)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0, options: super::CreationCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderAsync)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::Interface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetItemAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    pub fn GetIndexedStateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexedStateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQuery)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQuery)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQuery(&self) -> ::windows_core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows_core::IntoParam<'a, super::Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreQueryOptionsSupported)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFolderQuerySupported)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFileQuerySupported)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, desiredname: Param0, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenameAsync)(::windows_core::Interface::as_raw(this), desiredname.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Attributes(&self) -> ::windows_core::Result<super::FileAttributes> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::FileAttributes>::zeroed();
            (::windows_core::Interface::vtable(this).Attributes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DateCreated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOfType)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetParentAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn IsEqual<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, item: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEqual)(::windows_core::Interface::as_raw(this), item.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BasicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::windows_core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::windows_core::Interface::as_raw(this), mode, requestedsize, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), mode, requestedsize, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn DisplayType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FolderRelativeId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
    pub fn Provider(&self) -> ::windows_core::Result<super::StorageProvider> {
        let this = &::windows_core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Provider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FolderInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderInformation {}
impl ::core::fmt::Debug for FolderInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FolderInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FolderInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FolderInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows_core::GUID = <IStorageItemInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
impl ::core::convert::From<FolderInformation> for ::windows_core::IUnknown {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows_core::IUnknown {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FolderInformation> for ::windows_core::IInspectable {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows_core::IInspectable {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFolder> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFolder> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFolder> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFolder> {
        ::core::convert::TryInto::<super::IStorageFolder>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFolder2> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFolder2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageFolder2> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageFolder2> {
        ::core::convert::TryInto::<super::IStorageFolder2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<&FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows_core::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows_core::IntoParam<'a, super::Search::IStorageFolderQueryOperations> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Search::IStorageFolderQueryOperations> {
        ::core::convert::TryInto::<super::Search::IStorageFolderQueryOperations>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem> {
        ::core::convert::TryInto::<super::IStorageItem>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem2> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItem2> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItem2> {
        ::core::convert::TryInto::<super::IStorageItem2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for IStorageItemInformation {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for IStorageItemInformation {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemInformation> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemInformation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemInformation> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemInformation> {
        ::core::convert::TryInto::<IStorageItemInformation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemProperties> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemProperties> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemProperties> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemProperties> {
        ::core::convert::TryInto::<super::IStorageItemProperties>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IStorageItemPropertiesWithProvider> for &FolderInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<super::IStorageItemPropertiesWithProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x401d88be_960f_4d6d_a7d0_1a3861e76c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
    pub GetVirtualizedItemsVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVirtualizedFilesVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVirtualizedFoldersVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactoryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileInformationFactoryFactory {
    type Vtable = IFileInformationFactoryFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84ea0e7d_e4a2_4f00_8afa_af5e0f826bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows_core::RawPtr, mode: super::FileProperties::ThumbnailMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithMode: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows_core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSize: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows_core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptions: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptionsAndFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: ::windows_core::RawPtr, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptionsAndFlags: usize,
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct IStorageItemInformation(::windows_core::IUnknown);
impl IStorageItemInformation {
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BasicProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PropertiesUpdated)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_BulkAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertiesUpdated)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows_core::IUnknown {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows_core::IUnknown {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStorageItemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStorageItemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows_core::IInspectable {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows_core::IInspectable {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IStorageItemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IStorageItemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemInformation {}
impl ::core::fmt::Debug for IStorageItemInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IStorageItemInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{87a5cb8b-8972-4f40-8de0-d86fb179d8fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IStorageItemInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87a5cb8b_8972_4f40_8de0_d86fb179d8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    ImageProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub DocumentProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    DocumentProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub BasicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    BasicProperties: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub ThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertiesUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertiesUpdated: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
