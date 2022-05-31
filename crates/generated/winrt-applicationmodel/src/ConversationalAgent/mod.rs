#[repr(transparent)]
pub struct ActivationSignalDetectionConfiguration(::windows_core::IUnknown);
impl ActivationSignalDetectionConfiguration {
    pub fn SignalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SignalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetEnabledAsync(&self, value: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetEnabledAsync)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn AvailabilityInfo(&self) -> ::windows_core::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailabilityInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DetectionConfigurationAvailabilityInfo>(result__)
        }
    }
    pub fn AvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModelData)(::windows_core::Interface::as_raw(this), datatype.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetModelDataAsync)(::windows_core::Interface::as_raw(this), datatype.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetModelDataType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetModelDataType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetModelDataTypeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetModelDataTypeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelData(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetModelData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetModelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IInputStream>>(result__)
        }
    }
    pub fn ClearModelData(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearModelData)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearModelDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClearModelDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn TrainingStepsCompleted(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TrainingStepsCompleted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TrainingStepsRemaining(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TrainingStepsRemaining)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TrainingDataFormat(&self) -> ::windows_core::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectionTrainingDataFormat>::zeroed();
            (::windows_core::Interface::vtable(this).TrainingDataFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionTrainingDataFormat>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingData<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows_core::Result<DetectionConfigurationTrainingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DetectionConfigurationTrainingStatus>::zeroed();
            (::windows_core::Interface::vtable(this).ApplyTrainingData)(::windows_core::Interface::as_raw(this), trainingdataformat, trainingdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<DetectionConfigurationTrainingStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingDataAsync<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ApplyTrainingDataAsync)(::windows_core::Interface::as_raw(this), trainingdataformat, trainingdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>(result__)
        }
    }
    pub fn ClearTrainingData(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearTrainingData)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearTrainingDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClearTrainingDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResult<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows_core::Result<ActivationSignalDetectionConfigurationSetModelDataResult> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectionConfigurationSetModelDataResult>::zeroed();
            (::windows_core::Interface::vtable(this).SetModelDataWithResult)(::windows_core::Interface::as_raw(this), datatype.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfigurationSetModelDataResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResultAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetModelDataWithResultAsync)(::windows_core::Interface::as_raw(this), datatype.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>(result__)
        }
    }
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetEnabledWithResultAsync)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>(result__)
        }
    }
    pub fn SetEnabledWithResult(&self, value: bool) -> ::windows_core::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectionConfigurationStateChangeResult>::zeroed();
            (::windows_core::Interface::vtable(this).SetEnabledWithResult)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfigurationStateChangeResult>(result__)
        }
    }
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).TrainingStepCompletionMaxAllowedTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfiguration {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration;{40d8be16-5217-581c-9ab2-ce9b2f2e8e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IActivationSignalDetectionConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows_core::IUnknown {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows_core::IInspectable {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ActivationSignalDetectionConfiguration> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ActivationSignalDetectionConfiguration) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ActivationSignalDetectionConfiguration> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfiguration {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfiguration {}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationResult(::windows_core::IUnknown);
impl ActivationSignalDetectionConfigurationCreationResult {
    pub fn Status(&self) -> ::windows_core::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectionConfigurationCreationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfigurationCreationStatus>(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfigurationCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfigurationCreationResult {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult;{4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
    const IID: ::windows_core::GUID = <IActivationSignalDetectionConfigurationCreationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows_core::IUnknown {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows_core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows_core::IInspectable {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows_core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SignalIdNotAvailable: Self = Self(1i32);
    pub const ModelIdNotSupported: Self = Self(2i32);
    pub const InvalidSignalId: Self = Self(3i32);
    pub const InvalidModelId: Self = Self(4i32);
    pub const InvalidDisplayName: Self = Self(5i32);
    pub const ConfigurationAlreadyExists: Self = Self(6i32);
    pub const CreationNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationCreationStatus {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectionConfigurationCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: Self = Self(0i32);
    pub const NotFound: Self = Self(1i32);
    pub const CurrentlyEnabled: Self = Self(2i32);
    pub const RemovalNotSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationRemovalResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationRemovalResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationRemovalResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectionConfigurationRemovalResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationRemovalResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationRemovalResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: Self = Self(0i32);
    pub const EmptyModelData: Self = Self(1i32);
    pub const UnsupportedFormat: Self = Self(2i32);
    pub const ConfigurationCurrentlyEnabled: Self = Self(3i32);
    pub const InvalidData: Self = Self(4i32);
    pub const SetModelDataNotSupported: Self = Self(5i32);
    pub const ConfigurationNotFound: Self = Self(6i32);
    pub const UnknownError: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationSetModelDataResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectionConfigurationSetModelDataResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationSetModelDataResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: Self = Self(0i32);
    pub const NoModelData: Self = Self(1i32);
    pub const ConfigurationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationStateChangeResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationStateChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectionConfigurationStateChangeResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: Self = Self(0i32);
    pub const Voice8kHz16BitMono: Self = Self(1i32);
    pub const Voice16kHz8BitMono: Self = Self(2i32);
    pub const Voice16kHz16BitMono: Self = Self(3i32);
    pub const VoiceOEMDefined: Self = Self(4i32);
    pub const Audio44kHz8BitMono: Self = Self(5i32);
    pub const Audio44kHz16BitMono: Self = Self(6i32);
    pub const Audio48kHz8BitMono: Self = Self(7i32);
    pub const Audio48kHz16BitMono: Self = Self(8i32);
    pub const AudioOEMDefined: Self = Self(9i32);
    pub const OtherOEMDefined: Self = Self(10i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionTrainingDataFormat {}
impl ::core::clone::Clone for ActivationSignalDetectionTrainingDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionTrainingDataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectionTrainingDataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionTrainingDataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionTrainingDataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ActivationSignalDetector(::windows_core::IUnknown);
impl ActivationSignalDetector {
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectorKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
    pub fn CanCreateConfigurations(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanCreateConfigurations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModelDataTypes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModelDataTypes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTrainingDataFormats(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTrainingDataFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPowerStates(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPowerStates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedModelIdsForSignalId)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedModelIdsForSignalIdAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>>(result__)
        }
    }
    pub fn CreateConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CreateConfiguration)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    pub fn CreateConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateConfigurationAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurations(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConfigurations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurationsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConfigurationsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>(result__)
        }
    }
    pub fn GetConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConfiguration)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
    pub fn GetConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConfigurationAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    pub fn RemoveConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConfiguration)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi()).ok() }
    }
    pub fn RemoveConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveConfigurationAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableModelIdsForSignalIdAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableModelIdsForSignalId)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn CreateConfigurationWithResultAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateConfigurationWithResultAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>(result__)
        }
    }
    pub fn CreateConfigurationWithResult<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows_core::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateConfigurationWithResult)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfigurationCreationResult>(result__)
        }
    }
    pub fn RemoveConfigurationWithResultAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveConfigurationWithResultAsync)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>(result__)
        }
    }
    pub fn RemoveConfigurationWithResult<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows_core::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectionConfigurationRemovalResult>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveConfigurationWithResult)(::windows_core::Interface::as_raw(this), signalid.into_param().abi(), modelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectionConfigurationRemovalResult>(result__)
        }
    }
    pub fn DetectorId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DetectorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetector {}
impl ::core::fmt::Debug for ActivationSignalDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector;{b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
    const IID: ::windows_core::GUID = <IActivationSignalDetector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows_core::IUnknown {
    fn from(value: ActivationSignalDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows_core::IUnknown {
    fn from(value: &ActivationSignalDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivationSignalDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows_core::IInspectable {
    fn from(value: ActivationSignalDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows_core::IInspectable {
    fn from(value: &ActivationSignalDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivationSignalDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetector {}
unsafe impl ::core::marker::Sync for ActivationSignalDetector {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: Self = Self(0i32);
    pub const AudioImpulse: Self = Self(1i32);
    pub const HardwareEvent: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorKind {}
impl ::core::clone::Clone for ActivationSignalDetectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: Self = Self(0i32);
    pub const ConnectedLowPower: Self = Self(1i32);
    pub const DisconnectedLowPower: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorPowerState {}
impl ::core::clone::Clone for ActivationSignalDetectorPowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorPowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationSignalDetectorPowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorPowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorPowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationKind {}
impl ::core::clone::Clone for ConversationalAgentActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: Self = Self(0i32);
    pub const AgentInactive: Self = Self(1i32);
    pub const ScreenNotAvailable: Self = Self(2i32);
    pub const AgentInterrupted: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationResult {}
impl ::core::clone::Clone for ConversationalAgentActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentActivationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ConversationalAgentDetectorManager(::windows_core::IUnknown);
impl ConversationalAgentDetectorManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllActivationSignalDetectors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllActivationSignalDetectorsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetActivationSignalDetectors)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetActivationSignalDetectorsAsync)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    pub fn GetActivationSignalDetectorFromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, detectorid: Param0) -> ::windows_core::Result<ActivationSignalDetector> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetActivationSignalDetectorFromId)(::windows_core::Interface::as_raw(this), detectorid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ActivationSignalDetector>(result__)
        }
    }
    pub fn GetActivationSignalDetectorFromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, detectorid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetActivationSignalDetectorFromIdAsync)(::windows_core::Interface::as_raw(this), detectorid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ActivationSignalDetector>>(result__)
        }
    }
    pub fn Default() -> ::windows_core::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Default)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentDetectorManager>(result__)
        })
    }
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConversationalAgentDetectorManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentDetectorManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentDetectorManager {}
impl ::core::fmt::Debug for ConversationalAgentDetectorManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentDetectorManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager;{de94fbb0-597a-5df8-8cfb-9dbb583ba3ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentDetectorManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentDetectorManager {}
unsafe impl ::core::marker::Sync for ConversationalAgentDetectorManager {}
#[repr(transparent)]
pub struct ConversationalAgentSession(::windows_core::IUnknown);
impl ConversationalAgentSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SessionInterrupted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SessionInterrupted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSessionInterrupted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSessionInterrupted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SignalDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SignalDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSignalDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSignalDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SystemStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SystemStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSystemStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSystemStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AgentState(&self) -> ::windows_core::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentState>::zeroed();
            (::windows_core::Interface::vtable(this).AgentState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentState>(result__)
        }
    }
    pub fn Signal(&self) -> ::windows_core::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Signal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentSignal>(result__)
        }
    }
    pub fn IsIndicatorLightAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIndicatorLightAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsScreenAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScreenAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsUserAuthenticated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsUserAuthenticated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVoiceActivationAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVoiceActivationAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInterruptible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInterruptible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInterrupted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInterrupted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInterruptibleAsync)(::windows_core::Interface::as_raw(this), interruptible, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestInterruptible(&self, interruptible: bool) -> ::windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentSessionUpdateResponse>::zeroed();
            (::windows_core::Interface::vtable(this).RequestInterruptible)(::windows_core::Interface::as_raw(this), interruptible, result__.as_mut_ptr()).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAgentStateChangeAsync)(::windows_core::Interface::as_raw(this), state, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentSessionUpdateResponse>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAgentStateChange)(::windows_core::Interface::as_raw(this), state, result__.as_mut_ptr()).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    pub fn RequestForegroundActivationAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestForegroundActivationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    pub fn RequestForegroundActivation(&self) -> ::windows_core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentSessionUpdateResponse>::zeroed();
            (::windows_core::Interface::vtable(this).RequestForegroundActivation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    pub fn GetAudioClientAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioClientAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::IInspectable>>(result__)
        }
    }
    pub fn GetAudioClient(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioClient)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNodeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_media::Audio::AudioDeviceInputNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioDeviceInputNodeAsync)(::windows_core::Interface::as_raw(this), graph.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_media::Audio::AudioDeviceInputNode>>(result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNode<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows_core::Result<::winrt_media::Audio::AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAudioDeviceInputNode)(::windows_core::Interface::as_raw(this), graph.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_media::Audio::AudioDeviceInputNode>(result__)
        }
    }
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioCaptureDeviceIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GetAudioCaptureDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioCaptureDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetAudioRenderDeviceIdAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioRenderDeviceIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GetAudioRenderDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioRenderDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetSignalModelIdAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSignalModelIdAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetSignalModelId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetSignalModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetSignalModelIdAsync)(::windows_core::Interface::as_raw(this), signalmodelid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SetSignalModelId)(::windows_core::Interface::as_raw(this), signalmodelid, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedSignalModelIdsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedSignalModelIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestActivationAsync)(::windows_core::Interface::as_raw(this), activationkind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ConversationalAgentActivationResult>>(result__)
        }
    }
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows_core::Result<ConversationalAgentActivationResult> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentActivationResult>::zeroed();
            (::windows_core::Interface::vtable(this).RequestActivation)(::windows_core::Interface::as_raw(this), activationkind, result__.as_mut_ptr()).from_abi::<ConversationalAgentActivationResult>(result__)
        }
    }
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetSupportLockScreenActivationAsync)(::windows_core::Interface::as_raw(this), lockscreenactivationsupported, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportLockScreenActivation)(::windows_core::Interface::as_raw(this), lockscreenactivationsupported).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisites(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMissingPrerequisites)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisitesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMissingPrerequisitesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>(result__)
        }
    }
    pub fn GetCurrentSessionAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSessionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ConversationalAgentSession>>(result__)
        })
    }
    pub fn GetCurrentSessionSync() -> ::windows_core::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSessionSync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentSession>(result__)
        })
    }
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConversationalAgentSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSession {}
impl ::core::fmt::Debug for ConversationalAgentSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSession").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession;{daaae09a-b7ba-57e5-ad13-df520f9b6fa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConversationalAgentSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ConversationalAgentSession) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConversationalAgentSession> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ConversationalAgentSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSession {}
unsafe impl ::core::marker::Sync for ConversationalAgentSession {}
#[repr(transparent)]
pub struct ConversationalAgentSessionInterruptedEventArgs(::windows_core::IUnknown);
impl ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSessionInterruptedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSessionInterruptedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSessionInterruptedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionInterruptedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs;{9766591f-f63d-5d3e-9bf2-bd0760552686})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentSessionInterruptedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentSessionUpdateResponse {}
impl ::core::clone::Clone for ConversationalAgentSessionUpdateResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSessionUpdateResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentSessionUpdateResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSessionUpdateResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionUpdateResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ConversationalAgentSignal(::windows_core::IUnknown);
impl ConversationalAgentSignal {
    pub fn IsSignalVerificationRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSignalVerificationRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSignalVerificationRequired)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SignalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSignalId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignalName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SignalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSignalName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignalContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SignalContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetSignalContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalContext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignalStart(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SignalStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetSignalStart<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalStart)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignalEnd(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).SignalEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetSignalEnd<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalEnd)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DetectorId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DetectorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DetectorKind(&self) -> ::windows_core::Result<ActivationSignalDetectorKind> {
        let this = &::windows_core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationSignalDetectorKind>::zeroed();
            (::windows_core::Interface::vtable(this).DetectorKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSignal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignal {}
impl ::core::fmt::Debug for ConversationalAgentSignal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignal").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal;{20ed25f7-b120-51f2-8603-265d6a47f232})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentSignal as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentSignal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentSignal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentSignal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentSignal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignal {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignal {}
#[repr(transparent)]
pub struct ConversationalAgentSignalDetectedEventArgs(::windows_core::IUnknown);
impl ConversationalAgentSignalDetectedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSignalDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignalDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignalDetectedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSignalDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignalDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs;{4d57eb8f-f88a-599b-91d3-d604876708bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentSignalDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignalDetectedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const Listening: Self = Self(2i32);
    pub const Working: Self = Self(3i32);
    pub const Speaking: Self = Self(4i32);
    pub const ListeningAndSpeaking: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentState {}
impl ::core::clone::Clone for ConversationalAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: Self = Self(0i32);
    pub const ScreenAvailability: Self = Self(1i32);
    pub const IndicatorLightAvailability: Self = Self(2i32);
    pub const VoiceActivationAvailability: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentSystemStateChangeType {}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSystemStateChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentSystemStateChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangedEventArgs(::windows_core::IUnknown);
impl ConversationalAgentSystemStateChangedEventArgs {
    pub fn SystemStateChangeType(&self) -> ::windows_core::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConversationalAgentSystemStateChangeType>::zeroed();
            (::windows_core::Interface::vtable(this).SystemStateChangeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConversationalAgentSystemStateChangeType>(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSystemStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSystemStateChangedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs;{1c2c6e3e-2785-59a7-8e71-38adeef79928})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IConversationalAgentSystemStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: Self = Self(0i32);
    pub const KnownAgents: Self = Self(1i32);
    pub const AgentAllowed: Self = Self(2i32);
    pub const AppCapability: Self = Self(3i32);
    pub const BackgroundTaskRegistration: Self = Self(4i32);
    pub const PolicyPermission: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentVoiceActivationPrerequisiteKind {}
impl ::core::clone::Clone for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConversationalAgentVoiceActivationPrerequisiteKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentVoiceActivationPrerequisiteKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: Self = Self(0i32);
    pub const Permission: Self = Self(1i32);
    pub const LockScreenPermission: Self = Self(2i32);
}
impl ::core::marker::Copy for DetectionConfigurationAvailabilityChangeKind {}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationAvailabilityChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DetectionConfigurationAvailabilityChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(::windows_core::IUnknown);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DetectionConfigurationAvailabilityChangeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DetectionConfigurationAvailabilityChangeKind>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs;{5129c9fb-4be8-5f14-af2b-88d62b1b4462})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDetectionConfigurationAvailabilityChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityInfo(::windows_core::IUnknown);
impl DetectionConfigurationAvailabilityInfo {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasSystemResourceAccess(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasSystemResourceAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasPermission(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasPermission)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasLockScreenPermission(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasLockScreenPermission)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnavailableSystemResources(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &::windows_core::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnavailableSystemResources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SignalDetectorResourceKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityInfo {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo;{b5affeb0-40f0-5398-b838-91979c2c6208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
    const IID: ::windows_core::GUID = <IDetectionConfigurationAvailabilityInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows_core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows_core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows_core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows_core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const VoiceTooQuiet: Self = Self(2i32);
    pub const VoiceTooLoud: Self = Self(3i32);
    pub const VoiceTooFast: Self = Self(4i32);
    pub const VoiceTooSlow: Self = Self(5i32);
    pub const VoiceQualityProblem: Self = Self(6i32);
    pub const TrainingSystemInternalError: Self = Self(7i32);
    pub const TrainingTimedOut: Self = Self(8i32);
    pub const ConfigurationNotFound: Self = Self(9i32);
}
impl ::core::marker::Copy for DetectionConfigurationTrainingStatus {}
impl ::core::clone::Clone for DetectionConfigurationTrainingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationTrainingStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DetectionConfigurationTrainingStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationTrainingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationTrainingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SetEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AvailabilityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataAsync: usize,
    pub GetModelDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetModelDataTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelDataAsync: usize,
    pub ClearModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrainingStepsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TrainingStepsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TrainingDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows_core::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingDataAsync: usize,
    pub ClearTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationSignalDetectionConfiguration2 {
    type Vtable = IActivationSignalDetectionConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71d9b022_562c_57ce_a78b_8b4ff0145bab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResult: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResultAsync: usize,
    pub SetEnabledWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEnabledWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows_core::HRESULT,
    pub TrainingStepCompletionMaxAllowedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfigurationCreationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows_core::HRESULT,
    pub CanCreateConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModelDataTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModelDataTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTrainingDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTrainingDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPowerStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPowerStates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalIdAsync: usize,
    pub CreateConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CreateConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurationsAsync: usize,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationSignalDetector2 {
    type Vtable = IActivationSignalDetector2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7e2490a_baa5_59d2_85d1_ba42f7cf78c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalId: usize,
    pub CreateConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows_core::HRESULT,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectorsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectorsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentDetectorManager2 {
    type Vtable = IConversationalAgentDetectorManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84610f31_d7f3_52fe_9311_c9eb4e3eb30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetActivationSignalDetectorFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetActivationSignalDetectorFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentDetectorManagerStatics {
    type Vtable = IConversationalAgentDetectorManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36a8d283_fa0e_5693_8489_0fb2f0ab40d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AgentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentState) -> ::windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsIndicatorLightAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsScreenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsUserAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVoiceActivationAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RequestInterruptibleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_core::HRESULT,
    pub RequestAgentStateChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAgentStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_core::HRESULT,
    pub RequestForegroundActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestForegroundActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_core::HRESULT,
    pub GetAudioClientAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAudioClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNodeAsync: usize,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNode: usize,
    pub GetAudioCaptureDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAudioCaptureDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAudioRenderDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAudioRenderDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSession2 {
    type Vtable = IConversationalAgentSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7a9fbf9_ac78_57ff_9596_acc7a1c9a607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows_core::HRESULT,
    pub SetSupportLockScreenActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSupportLockScreenActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisitesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisitesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionInterruptedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9766591f_f63d_5d3e_9bf2_bd0760552686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSessionStatics {
    type Vtable = IConversationalAgentSessionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa005166e_e954_576e_be04_11b8ed10f37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCurrentSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCurrentSessionSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20ed25f7_b120_51f2_8603_265d6a47f232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetSignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetSignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSignal2 {
    type Vtable = IConversationalAgentSignal2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0cc7ba9_9a7b_5c34_880e_b6146c904ecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DetectorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignalDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d57eb8f_f88a_599b_91d3_d604876708bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSystemStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SystemStateChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5affeb0_40f0_5398_b838_91979c2c6208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasSystemResourceAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasLockScreenPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDetectionConfigurationAvailabilityInfo2 {
    type Vtable = IDetectionConfigurationAvailabilityInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30e06433_38b3_5c4b_84c3_62b6e685b2ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UnavailableSystemResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnavailableSystemResources: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: Self = Self(0i32);
    pub const ParallelModelSupportForAgent: Self = Self(1i32);
    pub const ParallelSignalSupport: Self = Self(2i32);
    pub const ParallelSignalSupportForAgent: Self = Self(3i32);
    pub const DisplayOffSupport: Self = Self(4i32);
    pub const PluggedInPower: Self = Self(5i32);
    pub const Detector: Self = Self(6i32);
    pub const SupportedSleepState: Self = Self(7i32);
    pub const SupportedBatterySaverState: Self = Self(8i32);
    pub const ScreenAvailability: Self = Self(9i32);
    pub const InputHardware: Self = Self(10i32);
    pub const AcousticEchoCancellation: Self = Self(11i32);
    pub const ModelIdSupport: Self = Self(12i32);
    pub const DataChannel: Self = Self(13i32);
}
impl ::core::marker::Copy for SignalDetectorResourceKind {}
impl ::core::clone::Clone for SignalDetectorResourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SignalDetectorResourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SignalDetectorResourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SignalDetectorResourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalDetectorResourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
