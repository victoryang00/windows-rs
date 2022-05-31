#[cfg(feature = "Geofencing")]
pub mod Geofencing;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
}
impl ::core::marker::Copy for AltitudeReferenceSystem {}
impl ::core::clone::Clone for AltitudeReferenceSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AltitudeReferenceSystem {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AltitudeReferenceSystem {
    type Abi = Self;
}
impl ::core::fmt::Debug for AltitudeReferenceSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltitudeReferenceSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl ::core::marker::Copy for BasicGeoposition {}
impl ::core::clone::Clone for BasicGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BasicGeoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
unsafe impl ::windows_core::Abi for BasicGeoposition {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for BasicGeoposition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BasicGeoposition>()) == 0 }
    }
}
impl ::core::cmp::Eq for BasicGeoposition {}
impl ::core::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct CivicAddress(::windows_core::IUnknown);
impl CivicAddress {
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for CivicAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CivicAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CivicAddress {}
impl ::core::fmt::Debug for CivicAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CivicAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CivicAddress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.CivicAddress;{a8567a1a-64f4-4d48-bcea-f6b008eca34c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CivicAddress {
    type Vtable = ICivicAddress_Vtbl;
    const IID: ::windows_core::GUID = <ICivicAddress as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
impl ::core::convert::From<CivicAddress> for ::windows_core::IUnknown {
    fn from(value: CivicAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CivicAddress> for ::windows_core::IUnknown {
    fn from(value: &CivicAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CivicAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CivicAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CivicAddress> for ::windows_core::IInspectable {
    fn from(value: CivicAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CivicAddress> for ::windows_core::IInspectable {
    fn from(value: &CivicAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CivicAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CivicAddress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CivicAddress {}
unsafe impl ::core::marker::Sync for CivicAddress {}
#[repr(transparent)]
pub struct GeoboundingBox(::windows_core::IUnknown);
impl GeoboundingBox {
    pub fn NorthwestCorner(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BasicGeoposition>::zeroed();
            (::windows_core::Interface::vtable(this).NorthwestCorner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn SoutheastCorner(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BasicGeoposition>::zeroed();
            (::windows_core::Interface::vtable(this).SoutheastCorner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn Center(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BasicGeoposition>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn MinAltitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MinAltitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MaxAltitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAltitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>, Param1: ::windows_core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>, Param1: ::windows_core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReference)(::windows_core::Interface::as_raw(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>, Param1: ::windows_core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryCompute<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCompute)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryComputeWithAltitudeReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeWithAltitudeReference)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), altituderefsystem, result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), altituderefsystem, spatialreferenceid, result__.as_mut_ptr()).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeoshapeType>::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AltitudeReferenceSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeoboundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeoboundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeoboundingBox {}
impl ::core::fmt::Debug for GeoboundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoboundingBox").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeoboundingBox {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeoboundingBox;{0896c80b-274f-43da-9a06-cbfcdaeb4ec2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
    const IID: ::windows_core::GUID = <IGeoboundingBox as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
impl ::core::convert::From<GeoboundingBox> for ::windows_core::IUnknown {
    fn from(value: GeoboundingBox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeoboundingBox> for ::windows_core::IUnknown {
    fn from(value: &GeoboundingBox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeoboundingBox> for ::windows_core::IInspectable {
    fn from(value: GeoboundingBox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeoboundingBox> for ::windows_core::IInspectable {
    fn from(value: &GeoboundingBox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GeoboundingBox> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: GeoboundingBox) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GeoboundingBox> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: &GeoboundingBox) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for &GeoboundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GeoboundingBox {}
unsafe impl ::core::marker::Sync for GeoboundingBox {}
#[repr(transparent)]
pub struct Geocircle(::windows_core::IUnknown);
impl Geocircle {
    pub fn Center(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BasicGeoposition>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn Radius(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Radius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), position.into_param().abi(), radius, result__.as_mut_ptr()).from_abi::<Geocircle>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), position.into_param().abi(), radius, altitudereferencesystem, result__.as_mut_ptr()).from_abi::<Geocircle>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows_core::Interface::as_raw(this), position.into_param().abi(), radius, altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi::<Geocircle>(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeoshapeType>::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AltitudeReferenceSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geocircle, IGeocircleFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geocircle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geocircle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocircle {}
impl ::core::fmt::Debug for Geocircle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocircle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geocircle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocircle;{39e45843-a7f9-4e63-92a7-ba0c28d124b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geocircle {
    type Vtable = IGeocircle_Vtbl;
    const IID: ::windows_core::GUID = <IGeocircle as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
impl ::core::convert::From<Geocircle> for ::windows_core::IUnknown {
    fn from(value: Geocircle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geocircle> for ::windows_core::IUnknown {
    fn from(value: &Geocircle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geocircle> for ::windows_core::IInspectable {
    fn from(value: Geocircle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geocircle> for ::windows_core::IInspectable {
    fn from(value: &Geocircle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Geocircle> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: Geocircle) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geocircle> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: &Geocircle) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for &Geocircle {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geocircle {}
unsafe impl ::core::marker::Sync for Geocircle {}
#[repr(transparent)]
pub struct Geocoordinate(::windows_core::IUnknown);
impl Geocoordinate {
    #[cfg(feature = "winrt-")]
    pub fn Latitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Latitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Longitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Longitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Altitude(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Altitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn Accuracy(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Accuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AltitudeAccuracy(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn Heading(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Heading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn Speed(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Speed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Point(&self) -> ::windows_core::Result<Geopoint> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geopoint>(result__)
        }
    }
    pub fn PositionSource(&self) -> ::windows_core::Result<PositionSource> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PositionSource>::zeroed();
            (::windows_core::Interface::vtable(this).PositionSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PositionSource>(result__)
        }
    }
    pub fn SatelliteData(&self) -> ::windows_core::Result<GeocoordinateSatelliteData> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SatelliteData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeocoordinateSatelliteData>(result__)
        }
    }
    pub fn PositionSourceTimestamp(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionSourceTimestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn IsRemoteSource(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for Geocoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geocoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocoordinate {}
impl ::core::fmt::Debug for Geocoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocoordinate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geocoordinate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocoordinate;{ee21a3aa-976a-4c70-803d-083ea55bcbc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
    const IID: ::windows_core::GUID = <IGeocoordinate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
impl ::core::convert::From<Geocoordinate> for ::windows_core::IUnknown {
    fn from(value: Geocoordinate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geocoordinate> for ::windows_core::IUnknown {
    fn from(value: &Geocoordinate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geocoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geocoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geocoordinate> for ::windows_core::IInspectable {
    fn from(value: Geocoordinate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geocoordinate> for ::windows_core::IInspectable {
    fn from(value: &Geocoordinate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geocoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geocoordinate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geocoordinate {}
unsafe impl ::core::marker::Sync for Geocoordinate {}
#[repr(transparent)]
pub struct GeocoordinateSatelliteData(::windows_core::IUnknown);
impl GeocoordinateSatelliteData {
    pub fn PositionDilutionOfPrecision(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionDilutionOfPrecision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn HorizontalDilutionOfPrecision(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalDilutionOfPrecision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn VerticalDilutionOfPrecision(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalDilutionOfPrecision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn GeometricDilutionOfPrecision(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GeometricDilutionOfPrecision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn TimeDilutionOfPrecision(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TimeDilutionOfPrecision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for GeocoordinateSatelliteData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeocoordinateSatelliteData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeocoordinateSatelliteData {}
impl ::core::fmt::Debug for GeocoordinateSatelliteData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeocoordinateSatelliteData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeocoordinateSatelliteData;{c32a74d9-2608-474c-912c-06dd490f4af7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
    const IID: ::windows_core::GUID = <IGeocoordinateSatelliteData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
impl ::core::convert::From<GeocoordinateSatelliteData> for ::windows_core::IUnknown {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeocoordinateSatelliteData> for ::windows_core::IUnknown {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeocoordinateSatelliteData> for ::windows_core::IInspectable {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeocoordinateSatelliteData> for ::windows_core::IInspectable {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeocoordinateSatelliteData {}
unsafe impl ::core::marker::Sync for GeocoordinateSatelliteData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for GeolocationAccessStatus {}
impl ::core::clone::Clone for GeolocationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeolocationAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeolocationAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeolocationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Geolocator(::windows_core::IUnknown);
impl Geolocator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geolocator, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DesiredAccuracy(&self) -> ::windows_core::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PositionAccuracy>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAccuracy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PositionAccuracy>(result__)
        }
    }
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAccuracy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MovementThreshold(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MovementThreshold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMovementThreshold(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMovementThreshold)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LocationStatus(&self) -> ::windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PositionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).LocationStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PositionStatus>(result__)
        }
    }
    pub fn GetGeopositionAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    pub fn GetGeopositionAsyncWithAgeAndTimeout<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, maximumage: Param0, timeout: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionAsyncWithAgeAndTimeout)(::windows_core::Interface::as_raw(this), maximumage.into_param().abi(), timeout.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    pub fn PositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePositionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AllowFallbackToConsentlessPositions(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGeolocator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AllowFallbackToConsentlessPositions)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<GeolocationAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetGeopositionHistoryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(starttime: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionHistoryAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetGeopositionHistoryWithDurationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(starttime: Param0, duration: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionHistoryWithDurationAsync)(::windows_core::Interface::as_raw(this), starttime.into_param().abi(), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    pub fn IsDefaultGeopositionRecommended() -> ::windows_core::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDefaultGeopositionRecommended)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetDefaultGeoposition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<BasicGeoposition>>>(value: Param0) -> ::windows_core::Result<()> {
        Self::IGeolocatorStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetDefaultGeoposition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn DefaultGeoposition() -> ::windows_core::Result<::winrt_foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultGeoposition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<BasicGeoposition>>(result__)
        })
    }
    pub fn DesiredAccuracyInMeters(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAccuracyInMeters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetDesiredAccuracyInMeters<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAccuracyInMeters)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geolocator, IGeolocatorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geolocator, IGeolocatorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geolocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geolocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geolocator {}
impl ::core::fmt::Debug for Geolocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geolocator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geolocator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geolocator;{a9c3bf62-4524-4989-8aa9-de019d2e551f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geolocator {
    type Vtable = IGeolocator_Vtbl;
    const IID: ::windows_core::GUID = <IGeolocator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
impl ::core::convert::From<Geolocator> for ::windows_core::IUnknown {
    fn from(value: Geolocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geolocator> for ::windows_core::IUnknown {
    fn from(value: &Geolocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geolocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geolocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geolocator> for ::windows_core::IInspectable {
    fn from(value: Geolocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geolocator> for ::windows_core::IInspectable {
    fn from(value: &Geolocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geolocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geolocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geolocator {}
unsafe impl ::core::marker::Sync for Geolocator {}
#[repr(transparent)]
pub struct Geopath(::windows_core::IUnknown);
impl Geopath {
    #[cfg(feature = "winrt-foundation")]
    pub fn Positions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Positions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<BasicGeoposition>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows_core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), result__.as_mut_ptr()).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReference)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), altitudereferencesystem, result__.as_mut_ptr()).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), positions.into_param().abi(), altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi::<Geopath>(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeoshapeType>::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AltitudeReferenceSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geopath, IGeopathFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geopath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geopath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopath {}
impl ::core::fmt::Debug for Geopath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopath").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geopath {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopath;{e53fd7b9-2da4-4714-a652-de8593289898})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geopath {
    type Vtable = IGeopath_Vtbl;
    const IID: ::windows_core::GUID = <IGeopath as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
impl ::core::convert::From<Geopath> for ::windows_core::IUnknown {
    fn from(value: Geopath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geopath> for ::windows_core::IUnknown {
    fn from(value: &Geopath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geopath> for ::windows_core::IInspectable {
    fn from(value: Geopath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geopath> for ::windows_core::IInspectable {
    fn from(value: &Geopath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Geopath> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: Geopath) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopath> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: &Geopath) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for &Geopath {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geopath {}
unsafe impl ::core::marker::Sync for Geopath {}
#[repr(transparent)]
pub struct Geopoint(::windows_core::IUnknown);
impl Geopoint {
    pub fn Position(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BasicGeoposition>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), position.into_param().abi(), result__.as_mut_ptr()).from_abi::<Geopoint>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), position.into_param().abi(), altitudereferencesystem, result__.as_mut_ptr()).from_abi::<Geopoint>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows_core::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows_core::Interface::as_raw(this), position.into_param().abi(), altitudereferencesystem, spatialreferenceid, result__.as_mut_ptr()).from_abi::<Geopoint>(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeoshapeType>::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AltitudeReferenceSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Geopoint, IGeopointFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Geopoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geopoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopoint {}
impl ::core::fmt::Debug for Geopoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geopoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopoint;{6bfa00eb-e56e-49bb-9caf-cbaa78a8bcef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geopoint {
    type Vtable = IGeopoint_Vtbl;
    const IID: ::windows_core::GUID = <IGeopoint as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
impl ::core::convert::From<Geopoint> for ::windows_core::IUnknown {
    fn from(value: Geopoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geopoint> for ::windows_core::IUnknown {
    fn from(value: &Geopoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geopoint> for ::windows_core::IInspectable {
    fn from(value: Geopoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geopoint> for ::windows_core::IInspectable {
    fn from(value: &Geopoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Geopoint> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: Geopoint) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopoint> for IGeoshape {
    type Error = ::windows_core::Error;
    fn try_from(value: &Geopoint) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGeoshape> for &Geopoint {
    fn into_param(self) -> ::windows_core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geopoint {}
unsafe impl ::core::marker::Sync for Geopoint {}
#[repr(transparent)]
pub struct Geoposition(::windows_core::IUnknown);
impl Geoposition {
    pub fn Coordinate(&self) -> ::windows_core::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Coordinate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geocoordinate>(result__)
        }
    }
    pub fn CivicAddress(&self) -> ::windows_core::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CivicAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CivicAddress>(result__)
        }
    }
    pub fn VenueData(&self) -> ::windows_core::Result<VenueData> {
        let this = &::windows_core::Interface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VenueData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VenueData>(result__)
        }
    }
}
impl ::core::clone::Clone for Geoposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geoposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geoposition {}
impl ::core::fmt::Debug for Geoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geoposition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geoposition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geoposition;{c18d0454-7d41-4ff7-a957-9dffb4ef7f5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geoposition {
    type Vtable = IGeoposition_Vtbl;
    const IID: ::windows_core::GUID = <IGeoposition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
impl ::core::convert::From<Geoposition> for ::windows_core::IUnknown {
    fn from(value: Geoposition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geoposition> for ::windows_core::IUnknown {
    fn from(value: &Geoposition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geoposition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geoposition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geoposition> for ::windows_core::IInspectable {
    fn from(value: Geoposition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geoposition> for ::windows_core::IInspectable {
    fn from(value: &Geoposition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geoposition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geoposition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geoposition {}
unsafe impl ::core::marker::Sync for Geoposition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl ::core::marker::Copy for GeoshapeType {}
impl ::core::clone::Clone for GeoshapeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeoshapeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeoshapeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeoshapeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoshapeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeoshapeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Geovisit(::windows_core::IUnknown);
impl Geovisit {
    pub fn Position(&self) -> ::windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geoposition>(result__)
        }
    }
    pub fn StateChange(&self) -> ::windows_core::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VisitStateChange>::zeroed();
            (::windows_core::Interface::vtable(this).StateChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VisitStateChange>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for Geovisit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Geovisit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geovisit {}
impl ::core::fmt::Debug for Geovisit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geovisit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Geovisit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geovisit;{b1877a76-9ef6-41ab-a0dd-793ece76e2de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Geovisit {
    type Vtable = IGeovisit_Vtbl;
    const IID: ::windows_core::GUID = <IGeovisit as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
impl ::core::convert::From<Geovisit> for ::windows_core::IUnknown {
    fn from(value: Geovisit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geovisit> for ::windows_core::IUnknown {
    fn from(value: &Geovisit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Geovisit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Geovisit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Geovisit> for ::windows_core::IInspectable {
    fn from(value: Geovisit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Geovisit> for ::windows_core::IInspectable {
    fn from(value: &Geovisit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Geovisit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Geovisit {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Geovisit {}
unsafe impl ::core::marker::Sync for Geovisit {}
#[repr(transparent)]
pub struct GeovisitMonitor(::windows_core::IUnknown);
impl GeovisitMonitor {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeovisitMonitor, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MonitoringScope(&self) -> ::windows_core::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<VisitMonitoringScope>::zeroed();
            (::windows_core::Interface::vtable(this).MonitoringScope)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<VisitMonitoringScope>(result__)
        }
    }
    pub fn Start(&self, value: VisitMonitoringScope) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn VisitStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VisitStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVisitStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVisitStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetLastReportAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLastReportAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Geovisit>>(result__)
        })
    }
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeovisitMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitMonitor {}
impl ::core::fmt::Debug for GeovisitMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeovisitMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitMonitor;{80118aaf-5944-4591-83c1-396647f54f2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IGeovisitMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
impl ::core::convert::From<GeovisitMonitor> for ::windows_core::IUnknown {
    fn from(value: GeovisitMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitMonitor> for ::windows_core::IUnknown {
    fn from(value: &GeovisitMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeovisitMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeovisitMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeovisitMonitor> for ::windows_core::IInspectable {
    fn from(value: GeovisitMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitMonitor> for ::windows_core::IInspectable {
    fn from(value: &GeovisitMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeovisitMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeovisitMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeovisitMonitor {}
unsafe impl ::core::marker::Sync for GeovisitMonitor {}
#[repr(transparent)]
pub struct GeovisitStateChangedEventArgs(::windows_core::IUnknown);
impl GeovisitStateChangedEventArgs {
    pub fn Visit(&self) -> ::windows_core::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Visit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geovisit>(result__)
        }
    }
}
impl ::core::clone::Clone for GeovisitStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitStateChangedEventArgs {}
impl ::core::fmt::Debug for GeovisitStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitStateChangedEventArgs;{ceb4d1ff-8b53-4968-beed-4cecd029ce15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGeovisitStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
impl ::core::convert::From<GeovisitStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitStateChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeovisitStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitStateChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeovisitStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GeovisitStateChangedEventArgs {}
#[repr(transparent)]
pub struct GeovisitTriggerDetails(::windows_core::IUnknown);
impl GeovisitTriggerDetails {
    #[cfg(feature = "winrt-foundation")]
    pub fn ReadReports(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Geovisit>>(result__)
        }
    }
}
impl ::core::clone::Clone for GeovisitTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeovisitTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitTriggerDetails {}
impl ::core::fmt::Debug for GeovisitTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitTriggerDetails;{ea770d9e-d1c9-454b-99b7-b2f8cdd2482f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IGeovisitTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
impl ::core::convert::From<GeovisitTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: GeovisitTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeovisitTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeovisitTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: GeovisitTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeovisitTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeovisitTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeovisitTriggerDetails {}
unsafe impl ::core::marker::Sync for GeovisitTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICivicAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICivicAddress {
    type Vtable = ICivicAddress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddress_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBox_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NorthwestCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBoxFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBoxFactory {
    type Vtable = IGeoboundingBoxFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4dfba589_0411_4abc_b3b5_5bbccb57d98c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoboundingBoxStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBoxStatics {
    type Vtable = IGeoboundingBoxStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67b80708_e61a_4cd0_841b_93233792b5ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub TryCompute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryCompute: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryComputeWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, altituderefsystem: AltitudeReferenceSystem, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryComputeWithAltitudeReference: usize,
    #[cfg(feature = "winrt-foundation")]
    pub TryComputeWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryComputeWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocircle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocircle {
    type Vtable = IGeocircle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircle_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocircleFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocircleFactory {
    type Vtable = IGeocircleFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafd6531f_72b1_4f7d_87cc_4ed4c9849c05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircleFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Latitude: usize,
    #[cfg(feature = "winrt-")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Longitude: usize,
    #[cfg(feature = "winrt-")]
    pub Altitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Altitude: usize,
    pub Accuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AltitudeAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Speed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc32a74d9_2608_474c_912c_06dd490f4af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HorizontalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VerticalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateSatelliteData2 {
    type Vtable = IGeocoordinateSatelliteData2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x761c8cfd_a19d_5a51_80f5_71676115483e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GeometricDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TimeDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPoint {
    type Vtable = IGeocoordinateWithPoint_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeea0525_d22c_4d46_b527_0b96066fc7db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPositionData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPositionData {
    type Vtable = IGeocoordinateWithPositionData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95e634be_dbd6_40ac_b8f2_a65c0340d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionSource) -> ::windows_core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithPositionSourceTimestamp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPositionSourceTimestamp {
    type Vtable = IGeocoordinateWithPositionSourceTimestamp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8543fc02_c9f1_4610_afe0_8bc3a6a87036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionSourceTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeocoordinateWithRemoteSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithRemoteSource {
    type Vtable = IGeocoordinateWithRemoteSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x397cebd7_ee38_5f3b_8900_c4a7bc9cf953);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRemoteSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocator {
    type Vtable = IGeolocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionAccuracy) -> ::windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PositionAccuracy) -> ::windows_core::HRESULT,
    pub MovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub LocationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows_core::HRESULT,
    pub GetGeopositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetGeopositionAsyncWithAgeAndTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumage: ::winrt_foundation::TimeSpan, timeout: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocator2 {
    type Vtable = IGeolocator2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1b42e6d_8891_43b4_ad36_27c6fe9a97b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorStatics {
    type Vtable = IGeolocatorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a8e7571_2df5_4591_9f87_eb5fd894e9b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetGeopositionHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetGeopositionHistoryAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetGeopositionHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: ::winrt_foundation::DateTime, duration: ::winrt_foundation::TimeSpan, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetGeopositionHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorStatics2 {
    type Vtable = IGeolocatorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x993011a2_fa1c_4631_a71d_0dbeb1250d9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDefaultGeopositionRecommended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeolocatorWithScalarAccuracy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorWithScalarAccuracy {
    type Vtable = IGeolocatorWithScalarAccuracy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96f5d3c1_b80f_460a_994d_a96c47a51aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopath(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopath {
    type Vtable = IGeopath_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe53fd7b9_2da4_4714_a652_de8593289898);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopath_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Positions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Positions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopathFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopathFactory {
    type Vtable = IGeopathFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27bea9c8_c7e7_4359_9b9b_fca3e05ef593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopathFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Create: usize,
    #[cfg(feature = "winrt-foundation")]
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateWithAltitudeReference: usize,
    #[cfg(feature = "winrt-foundation")]
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: ::windows_core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    CreateWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopoint {
    type Vtable = IGeopoint_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopoint_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeopointFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopointFactory {
    type Vtable = IGeopointFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb6b8d33_76bd_4e30_8af7_a844dc37b7a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopointFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoposition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoposition {
    type Vtable = IGeoposition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Coordinate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeoposition2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoposition2 {
    type Vtable = IGeoposition2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f62f697_8671_4b0d_86f8_474a8496187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VenueData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGeoshape(::windows_core::IUnknown);
impl IGeoshape {
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeoshapeType>::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AltitudeReferenceSystem>::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
}
impl ::core::convert::From<IGeoshape> for ::windows_core::IUnknown {
    fn from(value: IGeoshape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeoshape> for ::windows_core::IUnknown {
    fn from(value: &IGeoshape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGeoshape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGeoshape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGeoshape> for ::windows_core::IInspectable {
    fn from(value: IGeoshape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeoshape> for ::windows_core::IInspectable {
    fn from(value: &IGeoshape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGeoshape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGeoshape {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGeoshape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGeoshape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeoshape {}
impl ::core::fmt::Debug for IGeoshape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeoshape").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGeoshape {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c99ca2af-c729-43c1-8fab-d6dec914df7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGeoshape {
    type Vtable = IGeoshape_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99ca2af_c729_43c1_8fab_d6dec914df7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoshape_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GeoshapeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows_core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisit {
    type Vtable = IGeovisit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisit_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitStateChange) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80118aaf_5944_4591_83c1_396647f54f2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitMonitoringScope) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisitMonitoringScope) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitMonitorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitMonitorStatics {
    type Vtable = IGeovisitMonitorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcf976a7_bbf2_4cdd_95cf_554c82edfb87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetLastReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Visit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPositionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVenueData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVenueData {
    type Vtable = IVenueData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVenueData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for PositionAccuracy {}
impl ::core::clone::Clone for PositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PositionAccuracy {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionAccuracy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PositionAccuracy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PositionChangedEventArgs(::windows_core::IUnknown);
impl PositionChangedEventArgs {
    pub fn Position(&self) -> ::windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Geoposition>(result__)
        }
    }
}
impl ::core::clone::Clone for PositionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PositionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PositionChangedEventArgs {}
impl ::core::fmt::Debug for PositionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.PositionChangedEventArgs;{37859ce5-9d1e-46c5-bf3b-6ad8cac1a093})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPositionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
impl ::core::convert::From<PositionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PositionChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PositionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PositionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PositionChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PositionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PositionChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PositionChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
impl ::core::marker::Copy for PositionSource {}
impl ::core::clone::Clone for PositionSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PositionSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PositionSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for PositionStatus {}
impl ::core::clone::Clone for PositionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PositionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PositionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PositionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StatusChangedEventArgs(::windows_core::IUnknown);
impl StatusChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PositionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PositionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusChangedEventArgs {}
impl ::core::fmt::Debug for StatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.StatusChangedEventArgs;{3453d2da-8c93-4111-a205-9aecfc9be5c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
impl ::core::convert::From<StatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: StatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &StatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: StatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &StatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for StatusChangedEventArgs {}
#[repr(transparent)]
pub struct VenueData(::windows_core::IUnknown);
impl VenueData {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Level(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Level)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for VenueData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VenueData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VenueData {}
impl ::core::fmt::Debug for VenueData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VenueData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VenueData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.VenueData;{66f39187-60e3-4b2f-b527-4f53f1c3c677})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VenueData {
    type Vtable = IVenueData_Vtbl;
    const IID: ::windows_core::GUID = <IVenueData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
impl ::core::convert::From<VenueData> for ::windows_core::IUnknown {
    fn from(value: VenueData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VenueData> for ::windows_core::IUnknown {
    fn from(value: &VenueData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VenueData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VenueData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VenueData> for ::windows_core::IInspectable {
    fn from(value: VenueData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VenueData> for ::windows_core::IInspectable {
    fn from(value: &VenueData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VenueData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VenueData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VenueData {}
unsafe impl ::core::marker::Sync for VenueData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl ::core::marker::Copy for VisitMonitoringScope {}
impl ::core::clone::Clone for VisitMonitoringScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitMonitoringScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VisitMonitoringScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisitMonitoringScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitMonitoringScope").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl ::core::marker::Copy for VisitStateChange {}
impl ::core::clone::Clone for VisitStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitStateChange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VisitStateChange {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisitStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitStateChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VisitStateChange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
