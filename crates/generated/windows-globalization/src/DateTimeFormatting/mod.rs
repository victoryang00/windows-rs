#[repr(transparent)]
pub struct DateTimeFormatter(::windows_core::IUnknown);
impl DateTimeFormatter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Calendar(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Calendar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Clock(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Clock)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn Patterns(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Patterns)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Template(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Template)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Format<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IncludeYear(&self) -> ::windows_core::Result<YearFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<YearFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeYear)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<YearFormat>(result__)
        }
    }
    pub fn IncludeMonth(&self) -> ::windows_core::Result<MonthFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MonthFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeMonth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MonthFormat>(result__)
        }
    }
    pub fn IncludeDayOfWeek(&self) -> ::windows_core::Result<DayOfWeekFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DayOfWeekFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeDayOfWeek)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DayOfWeekFormat>(result__)
        }
    }
    pub fn IncludeDay(&self) -> ::windows_core::Result<DayFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DayFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeDay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DayFormat>(result__)
        }
    }
    pub fn IncludeHour(&self) -> ::windows_core::Result<HourFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HourFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeHour)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HourFormat>(result__)
        }
    }
    pub fn IncludeMinute(&self) -> ::windows_core::Result<MinuteFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<MinuteFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeMinute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MinuteFormat>(result__)
        }
    }
    pub fn IncludeSecond(&self) -> ::windows_core::Result<SecondFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SecondFormat>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeSecond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SecondFormat>(result__)
        }
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
    #[cfg(feature = "Foundation")]
    pub fn FormatUsingTimeZone<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, datetime: Param0, timezoneid: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDateTimeFormatter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FormatUsingTimeZone)(::windows_core::Interface::as_raw(this), datetime.into_param().abi(), timezoneid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateDateTimeFormatter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(formattemplate: Param0) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatter)(::windows_core::Interface::as_raw(this), formattemplate.into_param().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterLanguages<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(formattemplate: Param0, languages: Param1) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterLanguages)(::windows_core::Interface::as_raw(this), formattemplate.into_param().abi(), languages.into_param().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(formattemplate: Param0, languages: Param1, geographicregion: Param2, calendar: Param3, clock: Param4) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterContext)(::windows_core::Interface::as_raw(this), formattemplate.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterDate(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterDate)(::windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn CreateDateTimeFormatterTime(hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterTime)(::windows_core::Interface::as_raw(this), hourformat, minuteformat, secondformat, result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeLanguages<'a, Param7: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: Param7) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterDateTimeLanguages)(::windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeFormatterDateTimeContext<'a, Param7: ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param8: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param9: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param10: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: Param7, geographicregion: Param8, calendar: Param9, clock: Param10) -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeFormatterDateTimeContext)(::windows_core::Interface::as_raw(this), yearformat, monthformat, dayformat, dayofweekformat, hourformat, minuteformat, secondformat, languages.into_param().abi(), geographicregion.into_param().abi(), calendar.into_param().abi(), clock.into_param().abi(), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongDate() -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LongDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn LongTime() -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LongTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortDate() -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShortDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn ShortTime() -> ::windows_core::Result<DateTimeFormatter> {
        Self::IDateTimeFormatterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShortTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateTimeFormatter>(result__)
        })
    }
    pub fn IDateTimeFormatterFactory<R, F: FnOnce(&IDateTimeFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DateTimeFormatter, IDateTimeFormatterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDateTimeFormatterStatics<R, F: FnOnce(&IDateTimeFormatterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DateTimeFormatter, IDateTimeFormatterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DateTimeFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DateTimeFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DateTimeFormatter {}
impl ::core::fmt::Debug for DateTimeFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateTimeFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DateTimeFormatter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.DateTimeFormatting.DateTimeFormatter;{95eeca10-73e0-4e4b-a183-3d6ad0ba35ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DateTimeFormatter {
    type Vtable = IDateTimeFormatter_Vtbl;
    const IID: ::windows_core::GUID = <IDateTimeFormatter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DateTimeFormatter {
    const NAME: &'static str = "Windows.Globalization.DateTimeFormatting.DateTimeFormatter";
}
impl ::core::convert::From<DateTimeFormatter> for ::windows_core::IUnknown {
    fn from(value: DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows_core::IUnknown {
    fn from(value: &DateTimeFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DateTimeFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DateTimeFormatter> for ::windows_core::IInspectable {
    fn from(value: DateTimeFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DateTimeFormatter> for ::windows_core::IInspectable {
    fn from(value: &DateTimeFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DateTimeFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DateTimeFormatter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DateTimeFormatter {}
unsafe impl ::core::marker::Sync for DateTimeFormatter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for DayFormat {}
impl ::core::clone::Clone for DayFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DayFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DayFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for DayFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DayFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for DayOfWeekFormat {}
impl ::core::clone::Clone for DayOfWeekFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DayOfWeekFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DayOfWeekFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for DayOfWeekFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeekFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DayOfWeekFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.DayOfWeekFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for HourFormat {}
impl ::core::clone::Clone for HourFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HourFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HourFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for HourFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HourFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HourFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.HourFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeFormatter {
    type Vtable = IDateTimeFormatter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95eeca10_73e0_4e4b_a183_3d6ad0ba35ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Calendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Clock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Patterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Patterns: usize,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Format: usize,
    pub IncludeYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut YearFormat) -> ::windows_core::HRESULT,
    pub IncludeMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MonthFormat) -> ::windows_core::HRESULT,
    pub IncludeDayOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayOfWeekFormat) -> ::windows_core::HRESULT,
    pub IncludeDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DayFormat) -> ::windows_core::HRESULT,
    pub IncludeHour: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HourFormat) -> ::windows_core::HRESULT,
    pub IncludeMinute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MinuteFormat) -> ::windows_core::HRESULT,
    pub IncludeSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SecondFormat) -> ::windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeFormatter2 {
    type Vtable = IDateTimeFormatter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27c91a86_bdaa_4fd0_9e36_671d5aa5ee03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FormatUsingTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datetime: super::super::Foundation::DateTime, timezoneid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUsingTimeZone: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeFormatterFactory {
    type Vtable = IDateTimeFormatterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec8d8a53_1a2e_412d_8815_3b745fb1a2a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateDateTimeFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, languages: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattemplate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterContext: usize,
    pub CreateDateTimeFormatterDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateDateTimeFormatterTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: ::windows_core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, calendar: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, clock: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDateTimeFormatterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeFormatterStatics {
    type Vtable = IDateTimeFormatterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfcde7c0_df4c_4a2e_9012_f47daf3f1212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeFormatterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LongDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LongTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShortDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShortTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for MinuteFormat {}
impl ::core::clone::Clone for MinuteFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MinuteFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MinuteFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MinuteFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MinuteFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MinuteFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MinuteFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
    pub const Numeric: Self = Self(4i32);
}
impl ::core::marker::Copy for MonthFormat {}
impl ::core::clone::Clone for MonthFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MonthFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MonthFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for MonthFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MonthFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MonthFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.MonthFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondFormat {}
impl ::core::clone::Clone for SecondFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecondFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SecondFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecondFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SecondFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.SecondFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for YearFormat {}
impl ::core::clone::Clone for YearFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for YearFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for YearFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for YearFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("YearFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for YearFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Globalization.DateTimeFormatting.YearFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
