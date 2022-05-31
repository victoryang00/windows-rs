#[repr(transparent)]
pub struct CurrencyFormatter(::windows_core::IUnknown);
impl CurrencyFormatter {
    pub fn Currency(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Currency)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetCurrency<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrency)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Mode(&self) -> ::windows_core::Result<CurrencyFormatterMode> {
        let this = &::windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CurrencyFormatterMode>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CurrencyFormatterMode>(result__)
        }
    }
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyRoundingForCurrency)(::windows_core::Interface::as_raw(this), roundingalgorithm).ok() }
    }
    pub fn CreateCurrencyFormatterCode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(currencycode: Param0) -> ::windows_core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrencyFormatterCode)(::windows_core::Interface::as_raw(this), currencycode.into_param().abi(), result__.as_mut_ptr()).from_abi::<CurrencyFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCurrencyFormatterCodeContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(currencycode: Param0, languages: Param1, geographicregion: Param2) -> ::windows_core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrencyFormatterCodeContext)(::windows_core::Interface::as_raw(this), currencycode.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<CurrencyFormatter>(result__)
        })
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParseInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<i64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i64>>(result__)
        }
    }
    pub fn ParseUInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ParseDouble<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows_core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CurrencyFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrencyFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyFormatter {}
impl ::core::fmt::Debug for CurrencyFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.CurrencyFormatter;{11730ca5-4b00-41b2-b332-73b12a497d54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatter_Vtbl;
    const IID: ::windows_core::GUID = <ICurrencyFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
impl ::core::convert::From<CurrencyFormatter> for ::windows_core::IUnknown {
    fn from(value: CurrencyFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows_core::IUnknown {
    fn from(value: &CurrencyFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrencyFormatter> for ::windows_core::IInspectable {
    fn from(value: CurrencyFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows_core::IInspectable {
    fn from(value: &CurrencyFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CurrencyFormatter {}
unsafe impl ::core::marker::Sync for CurrencyFormatter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl ::core::marker::Copy for CurrencyFormatterMode {}
impl ::core::clone::Clone for CurrencyFormatterMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CurrencyFormatterMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CurrencyFormatterMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CurrencyFormatterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyFormatterMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DecimalFormatter(::windows_core::IUnknown);
impl DecimalFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DecimalFormatter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDecimalFormatter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows_core::Result<DecimalFormatter> {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDecimalFormatter)(::windows_core::Interface::as_raw(this), languages.into_param().abi(), geographicregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<DecimalFormatter>(result__)
        })
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParseInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<i64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i64>>(result__)
        }
    }
    pub fn ParseUInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ParseDouble<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows_core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DecimalFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DecimalFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DecimalFormatter {}
impl ::core::fmt::Debug for DecimalFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DecimalFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DecimalFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.DecimalFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DecimalFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
impl ::core::convert::From<DecimalFormatter> for ::windows_core::IUnknown {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows_core::IUnknown {
    fn from(value: &DecimalFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DecimalFormatter> for ::windows_core::IInspectable {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows_core::IInspectable {
    fn from(value: &DecimalFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DecimalFormatter {}
unsafe impl ::core::marker::Sync for DecimalFormatter {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatter {
    type Vtable = ICurrencyFormatter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCurrency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatter2 {
    type Vtable = ICurrencyFormatter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CurrencyFormatterMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CurrencyFormatterMode) -> ::windows_core::HRESULT,
    pub ApplyRoundingForCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundingalgorithm: RoundingAlgorithm) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatterFactory {
    type Vtable = ICurrencyFormatterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86c7537e_b938_4aa2_84b0_2c33dc5b1450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateCurrencyFormatterCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCurrencyFormatterCodeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCurrencyFormatterCodeContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDecimalFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDecimalFormatterFactory {
    type Vtable = IDecimalFormatterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDecimalFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDecimalFormatter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIncrementNumberRounder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIncrementNumberRounder {
    type Vtable = IIncrementNumberRounder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70a64ff8_66ab_4155_9da1_739e46764543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub Increment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberFormatter(::windows_core::IUnknown);
impl INumberFormatter {
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatter> for ::windows_core::IUnknown {
    fn from(value: INumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows_core::IUnknown {
    fn from(value: &INumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatter> for ::windows_core::IInspectable {
    fn from(value: INumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows_core::IInspectable {
    fn from(value: &INumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatter {}
impl ::core::fmt::Debug for INumberFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a5007c49-7676-4db7-8631-1b6ff265caa9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormatInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberFormatter2(::windows_core::IUnknown);
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatter2> for ::windows_core::IUnknown {
    fn from(value: INumberFormatter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows_core::IUnknown {
    fn from(value: &INumberFormatter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberFormatter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberFormatter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatter2> for ::windows_core::IInspectable {
    fn from(value: INumberFormatter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows_core::IInspectable {
    fn from(value: &INumberFormatter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberFormatter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberFormatter2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatter2 {}
impl ::core::fmt::Debug for INumberFormatter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatter2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberFormatter2 {
    type Vtable = INumberFormatter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormatInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberFormatterOptions(::windows_core::IUnknown);
impl INumberFormatterOptions {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows_core::IUnknown {
    fn from(value: INumberFormatterOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows_core::IUnknown {
    fn from(value: &INumberFormatterOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberFormatterOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberFormatterOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows_core::IInspectable {
    fn from(value: INumberFormatterOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows_core::IInspectable {
    fn from(value: &INumberFormatterOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberFormatterOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberFormatterOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatterOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatterOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatterOptions {}
impl ::core::fmt::Debug for INumberFormatterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatterOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{80332d21-aee1-4a39-baa2-07ed8c96daf6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberFormatterOptions {
    type Vtable = INumberFormatterOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IntegerDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetIntegerDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub FractionDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetFractionDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberParser(::windows_core::IUnknown);
impl INumberParser {
    pub fn ParseInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i64>>(result__)
        }
    }
    pub fn ParseUInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ParseDouble<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::convert::From<INumberParser> for ::windows_core::IUnknown {
    fn from(value: INumberParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberParser> for ::windows_core::IUnknown {
    fn from(value: &INumberParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberParser> for ::windows_core::IInspectable {
    fn from(value: INumberParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberParser> for ::windows_core::IInspectable {
    fn from(value: &INumberParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberParser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberParser {}
impl ::core::fmt::Debug for INumberParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberParser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberParser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e6659412-4a13-4a53-83a1-392fbe4cff9f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberParser {
    type Vtable = INumberParser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ParseUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ParseDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberRounder(::windows_core::IUnknown);
impl INumberRounder {
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::convert::From<INumberRounder> for ::windows_core::IUnknown {
    fn from(value: INumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows_core::IUnknown {
    fn from(value: &INumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberRounder> for ::windows_core::IInspectable {
    fn from(value: INumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows_core::IInspectable {
    fn from(value: &INumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberRounder {}
impl ::core::fmt::Debug for INumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberRounder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5473c375-38ed-4631-b80c-ef34fc48b7f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberRounder {
    type Vtable = INumberRounder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RoundInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RoundUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows_core::HRESULT,
    pub RoundInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows_core::HRESULT,
    pub RoundUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows_core::HRESULT,
    pub RoundSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RoundDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct INumberRounderOption(::windows_core::IUnknown);
impl INumberRounderOption {
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows_core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<INumberRounderOption> for ::windows_core::IUnknown {
    fn from(value: INumberRounderOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows_core::IUnknown {
    fn from(value: &INumberRounderOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for INumberRounderOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a INumberRounderOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberRounderOption> for ::windows_core::IInspectable {
    fn from(value: INumberRounderOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows_core::IInspectable {
    fn from(value: &INumberRounderOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for INumberRounderOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a INumberRounderOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberRounderOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberRounderOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberRounderOption {}
impl ::core::fmt::Debug for INumberRounderOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberRounderOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for INumberRounderOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3b088433-646f-4efe-8d48-66eb2e49e736}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for INumberRounderOption {
    type Vtable = INumberRounderOption_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b088433_646f_4efe_8d48_66eb2e49e736);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOption_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NumberRounder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNumberRounder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TranslateNumerals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslatorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemTranslatorFactory {
    type Vtable = INumeralSystemTranslatorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPercentFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPercentFormatterFactory {
    type Vtable = IPercentFormatterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7828aef_fed4_4018_a6e2_e09961e03765);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePercentFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePercentFormatter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPermilleFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPermilleFormatterFactory {
    type Vtable = IPermilleFormatterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePermilleFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePermilleFormatter: usize,
}
#[repr(transparent)]
pub struct ISignedZeroOption(::windows_core::IUnknown);
impl ISignedZeroOption {
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<ISignedZeroOption> for ::windows_core::IUnknown {
    fn from(value: ISignedZeroOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows_core::IUnknown {
    fn from(value: &ISignedZeroOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISignedZeroOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISignedZeroOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISignedZeroOption> for ::windows_core::IInspectable {
    fn from(value: ISignedZeroOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows_core::IInspectable {
    fn from(value: &ISignedZeroOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISignedZeroOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISignedZeroOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISignedZeroOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISignedZeroOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISignedZeroOption {}
impl ::core::fmt::Debug for ISignedZeroOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignedZeroOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fd1cdd31-0a3c-49c4-a642-96a1564f4f30}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISignedZeroOption {
    type Vtable = ISignedZeroOption_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOption_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsZeroSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsZeroSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignificantDigitsNumberRounder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISignificantDigitsNumberRounder {
    type Vtable = ISignificantDigitsNumberRounder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5941bca_6646_4913_8c76_1b191ff94dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISignificantDigitsOption(::windows_core::IUnknown);
impl ISignificantDigitsOption {
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows_core::IUnknown {
    fn from(value: ISignificantDigitsOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows_core::IUnknown {
    fn from(value: &ISignificantDigitsOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISignificantDigitsOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows_core::IInspectable {
    fn from(value: ISignificantDigitsOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows_core::IInspectable {
    fn from(value: &ISignificantDigitsOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISignificantDigitsOption {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISignificantDigitsOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISignificantDigitsOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISignificantDigitsOption {}
impl ::core::fmt::Debug for ISignificantDigitsOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignificantDigitsOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISignificantDigitsOption {
    type Vtable = ISignificantDigitsOption_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOption_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IncrementNumberRounder(::windows_core::IUnknown);
impl IncrementNumberRounder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IncrementNumberRounder, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RoundingAlgorithm(&self) -> ::windows_core::Result<RoundingAlgorithm> {
        let this = &::windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RoundingAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).RoundingAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoundingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Increment(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Increment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetIncrement(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncrement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for IncrementNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IncrementNumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IncrementNumberRounder {}
impl ::core::fmt::Debug for IncrementNumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IncrementNumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.IncrementNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IncrementNumberRounder {
    type Vtable = INumberRounder_Vtbl;
    const IID: ::windows_core::GUID = <INumberRounder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows_core::IUnknown {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows_core::IUnknown {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows_core::IInspectable {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows_core::IInspectable {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IncrementNumberRounder> for INumberRounder {
    type Error = ::windows_core::Error;
    fn try_from(value: IncrementNumberRounder) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IncrementNumberRounder> for INumberRounder {
    type Error = ::windows_core::Error;
    fn try_from(value: &IncrementNumberRounder) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounder> for IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounder> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounder> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounder> {
        ::core::convert::TryInto::<INumberRounder>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for IncrementNumberRounder {}
unsafe impl ::core::marker::Sync for IncrementNumberRounder {}
#[repr(transparent)]
pub struct NumeralSystemTranslator(::windows_core::IUnknown);
impl NumeralSystemTranslator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NumeralSystemTranslator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TranslateNumerals<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TranslateNumerals)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(languages: Param0) -> ::windows_core::Result<NumeralSystemTranslator> {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), languages.into_param().abi(), result__.as_mut_ptr()).from_abi::<NumeralSystemTranslator>(result__)
        })
    }
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NumeralSystemTranslator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NumeralSystemTranslator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NumeralSystemTranslator {}
impl ::core::fmt::Debug for NumeralSystemTranslator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NumeralSystemTranslator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.NumeralSystemTranslator;{28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_Vtbl;
    const IID: ::windows_core::GUID = <INumeralSystemTranslator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows_core::IUnknown {
    fn from(value: NumeralSystemTranslator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows_core::IUnknown {
    fn from(value: &NumeralSystemTranslator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NumeralSystemTranslator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows_core::IInspectable {
    fn from(value: NumeralSystemTranslator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows_core::IInspectable {
    fn from(value: &NumeralSystemTranslator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NumeralSystemTranslator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NumeralSystemTranslator {}
unsafe impl ::core::marker::Sync for NumeralSystemTranslator {}
#[repr(transparent)]
pub struct PercentFormatter(::windows_core::IUnknown);
impl PercentFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PercentFormatter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParseInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<i64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i64>>(result__)
        }
    }
    pub fn ParseUInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ParseDouble<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows_core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePercentFormatter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows_core::Result<PercentFormatter> {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePercentFormatter)(::windows_core::Interface::as_raw(this), languages.into_param().abi(), geographicregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<PercentFormatter>(result__)
        })
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PercentFormatter, IPercentFormatterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PercentFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PercentFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PercentFormatter {}
impl ::core::fmt::Debug for PercentFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PercentFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PercentFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PercentFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PercentFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
impl ::core::convert::From<PercentFormatter> for ::windows_core::IUnknown {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows_core::IUnknown {
    fn from(value: &PercentFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PercentFormatter> for ::windows_core::IInspectable {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows_core::IInspectable {
    fn from(value: &PercentFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PercentFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for &PercentFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PercentFormatter {}
unsafe impl ::core::marker::Sync for PercentFormatter {}
#[repr(transparent)]
pub struct PermilleFormatter(::windows_core::IUnknown);
impl PermilleFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PermilleFormatter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ParseInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<i64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i64>>(result__)
        }
    }
    pub fn ParseUInt<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn ParseDouble<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows_core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePermilleFormatter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows_core::Result<PermilleFormatter> {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePermilleFormatter)(::windows_core::Interface::as_raw(this), languages.into_param().abi(), geographicregion.into_param().abi(), result__.as_mut_ptr()).from_abi::<PermilleFormatter>(result__)
        })
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PermilleFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PermilleFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PermilleFormatter {}
impl ::core::fmt::Debug for PermilleFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PermilleFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PermilleFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PermilleFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PermilleFormatter {
    type Vtable = INumberFormatter_Vtbl;
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
impl ::core::convert::From<PermilleFormatter> for ::windows_core::IUnknown {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows_core::IUnknown {
    fn from(value: &PermilleFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PermilleFormatter> for ::windows_core::IInspectable {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows_core::IInspectable {
    fn from(value: &PermilleFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatter {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatter2> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberFormatterOptions> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberParser {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberParser> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberRounderOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounderOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignedZeroOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows_core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISignificantDigitsOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PermilleFormatter {}
unsafe impl ::core::marker::Sync for PermilleFormatter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: Self = Self(0i32);
    pub const RoundDown: Self = Self(1i32);
    pub const RoundUp: Self = Self(2i32);
    pub const RoundTowardsZero: Self = Self(3i32);
    pub const RoundAwayFromZero: Self = Self(4i32);
    pub const RoundHalfDown: Self = Self(5i32);
    pub const RoundHalfUp: Self = Self(6i32);
    pub const RoundHalfTowardsZero: Self = Self(7i32);
    pub const RoundHalfAwayFromZero: Self = Self(8i32);
    pub const RoundHalfToEven: Self = Self(9i32);
    pub const RoundHalfToOdd: Self = Self(10i32);
}
impl ::core::marker::Copy for RoundingAlgorithm {}
impl ::core::clone::Clone for RoundingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RoundingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RoundingAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for RoundingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoundingAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(::windows_core::IUnknown);
impl SignificantDigitsNumberRounder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SignificantDigitsNumberRounder, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn RoundingAlgorithm(&self) -> ::windows_core::Result<RoundingAlgorithm> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RoundingAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).RoundingAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoundingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignificantDigitsNumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignificantDigitsNumberRounder {}
impl ::core::fmt::Debug for SignificantDigitsNumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignificantDigitsNumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SignificantDigitsNumberRounder {
    type Vtable = INumberRounder_Vtbl;
    const IID: ::windows_core::GUID = <INumberRounder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows_core::IUnknown {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows_core::IUnknown {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows_core::IInspectable {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows_core::IInspectable {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SignificantDigitsNumberRounder> for INumberRounder {
    type Error = ::windows_core::Error;
    fn try_from(value: SignificantDigitsNumberRounder) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SignificantDigitsNumberRounder> for INumberRounder {
    type Error = ::windows_core::Error;
    fn try_from(value: &SignificantDigitsNumberRounder) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounder> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounder> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, INumberRounder> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows_core::Param<'a, INumberRounder> {
        ::core::convert::TryInto::<INumberRounder>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SignificantDigitsNumberRounder {}
unsafe impl ::core::marker::Sync for SignificantDigitsNumberRounder {}
