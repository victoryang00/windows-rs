#[cfg(feature = "UI_Accessibility")]
pub mod Accessibility;
#[cfg(feature = "UI_ApplicationSettings")]
pub mod ApplicationSettings;
#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Core")]
pub mod Core;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Notifications")]
pub mod Notifications;
#[cfg(feature = "UI_Popups")]
pub mod Popups;
#[cfg(feature = "UI_Shell")]
pub mod Shell;
#[cfg(feature = "UI_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_UIAutomation")]
pub mod UIAutomation;
#[cfg(feature = "UI_ViewManagement")]
pub mod ViewManagement;
#[cfg(feature = "UI_WebUI")]
pub mod WebUI;
#[cfg(feature = "UI_WindowManagement")]
pub mod WindowManagement;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl ::core::marker::Copy for Color {}
impl ::core::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Color").field("A", &self.A).field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
unsafe impl ::windows_core::Abi for Color {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for Color {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Color>()) == 0 }
    }
}
impl ::core::cmp::Eq for Color {}
impl ::core::default::Default for Color {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct ColorHelper(::windows_core::IUnknown);
impl ColorHelper {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows_core::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).FromArgb)(::windows_core::Interface::as_raw(this), a, r, g, b, result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn ToDisplayName<'a, Param0: ::windows_core::IntoParam<'a, Color>>(color: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToDisplayName)(::windows_core::Interface::as_raw(this), color.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorHelper, IColorHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics2<R, F: FnOnce(&IColorHelperStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorHelper, IColorHelperStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorHelper {}
impl ::core::fmt::Debug for ColorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.ColorHelper;{193cfbe7-65c7-4540-ad08-6283ba76879a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows_core::GUID = <IColorHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Windows.UI.ColorHelper";
}
impl ::core::convert::From<ColorHelper> for ::windows_core::IUnknown {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows_core::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorHelper> for ::windows_core::IInspectable {
    fn from(value: ColorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorHelper> for ::windows_core::IInspectable {
    fn from(value: &ColorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct Colors(::windows_core::IUnknown);
impl Colors {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AliceBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).AliceBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn AntiqueWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).AntiqueWhite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aqua() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Aqua)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Aquamarine() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Aquamarine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Azure() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Azure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Beige() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Beige)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Bisque() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Bisque)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Black() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Black)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlanchedAlmond() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).BlanchedAlmond)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Blue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Blue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BlueViolet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).BlueViolet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Brown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Brown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn BurlyWood() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).BurlyWood)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CadetBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).CadetBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chartreuse() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Chartreuse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Chocolate() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Chocolate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Coral() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Coral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn CornflowerBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).CornflowerBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cornsilk() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Cornsilk)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Crimson() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Crimson)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Cyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Cyan)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkCyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkCyan)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGoldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkGoldenrod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkKhaki() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkKhaki)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkMagenta() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkMagenta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOliveGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkOliveGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrange() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkOrange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkOrchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkOrchid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkRed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSalmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkSalmon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkSeaGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkSlateBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkSlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkSlateGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkTurquoise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DarkViolet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DarkViolet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DeepPink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DeepSkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DeepSkyBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DimGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DimGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn DodgerBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).DodgerBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Firebrick() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Firebrick)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn FloralWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).FloralWhite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn ForestGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).ForestGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Fuchsia() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Fuchsia)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gainsboro() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Gainsboro)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GhostWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).GhostWhite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gold() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Gold)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Goldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Goldenrod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Gray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Gray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Green() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Green)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn GreenYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).GreenYellow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Honeydew() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Honeydew)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn HotPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).HotPink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn IndianRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).IndianRed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Indigo() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Indigo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Ivory() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Ivory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Khaki() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Khaki)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lavender() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Lavender)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LavenderBlush() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LavenderBlush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LawnGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LawnGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LemonChiffon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LemonChiffon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCoral() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightCoral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightCyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightCyan)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGoldenrodYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightGoldenrodYellow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightPink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSalmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightSalmon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightSeaGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightSkyBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightSlateGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightSteelBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightSteelBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LightYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LightYellow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Lime() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Lime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn LimeGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).LimeGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Linen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Linen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Magenta() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Magenta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Maroon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Maroon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumAquamarine() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumAquamarine)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumOrchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumOrchid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumPurple() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumPurple)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumSeaGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumSlateBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumSpringGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumSpringGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumTurquoise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MediumVioletRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MediumVioletRed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MidnightBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MidnightBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MintCream() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MintCream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn MistyRose() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).MistyRose)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Moccasin() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Moccasin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn NavajoWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).NavajoWhite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Navy() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Navy)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OldLace() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).OldLace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Olive() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Olive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OliveDrab() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).OliveDrab)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orange() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Orange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn OrangeRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).OrangeRed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Orchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Orchid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGoldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PaleGoldenrod)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PaleGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PaleTurquoise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PaleVioletRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PaleVioletRed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PapayaWhip() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PapayaWhip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PeachPuff() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PeachPuff)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Peru() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Peru)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Pink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Pink)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Plum() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Plum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn PowderBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).PowderBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Purple() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Purple)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Red() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Red)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RosyBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).RosyBrown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn RoyalBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).RoyalBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SaddleBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SaddleBrown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Salmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Salmon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SandyBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SandyBrown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SeaGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SeaShell() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SeaShell)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Sienna() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Sienna)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Silver() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Silver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SkyBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SlateBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SlateGray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Snow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Snow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SpringGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SpringGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn SteelBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).SteelBlue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Tan)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Teal() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Teal)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Thistle() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Thistle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Tomato() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Tomato)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Transparent() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Transparent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Turquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Turquoise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Violet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Violet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Wheat() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Wheat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn White() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).White)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn WhiteSmoke() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).WhiteSmoke)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn Yellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).Yellow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn YellowGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Color>::zeroed();
            (::windows_core::Interface::vtable(this).YellowGreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Color>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Colors, IColorsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Colors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Colors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Colors {}
impl ::core::fmt::Debug for Colors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Colors").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Colors {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Colors;{9b8c9326-4ca6-4ce5-8994-9eff65cabdcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Colors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows_core::GUID = <IColors as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Colors {
    const NAME: &'static str = "Windows.UI.Colors";
}
impl ::core::convert::From<Colors> for ::windows_core::IUnknown {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows_core::IUnknown {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Colors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Colors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Colors> for ::windows_core::IInspectable {
    fn from(value: Colors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Colors> for ::windows_core::IInspectable {
    fn from(value: &Colors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Colors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Colors {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x193cfbe7_65c7_4540_ad08_6283ba76879a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8504dbea_fb6a_4144_a6c2_33499c9284f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromArgb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelperStatics2 {
    type Vtable = IColorHelperStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24d9af02_6eb0_4b94_855c_fcf0818d9a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: Color, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColors(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColors {
    type Vtable = IColors_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcff52e04_cca6_4614_a17e_754910c84a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AliceBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Aqua: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Azure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Beige: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Bisque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Black: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Blue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Brown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Coral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Crimson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Cyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DimGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Green: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub HotPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Indigo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Ivory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Khaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Lavender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Lime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Linen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Magenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Maroon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MintCream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Navy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OldLace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Olive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Orange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Orchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Peru: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Pink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Plum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Purple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Red: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Salmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Sienna: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Silver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Snow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Tan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Teal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Thistle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Tomato: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Transparent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Violet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Wheat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub White: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Yellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContentRoot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContentRoot_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIContext {
    type Vtable = IUIContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContext_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct UIContentRoot(::windows_core::IUnknown);
impl UIContentRoot {
    #[doc = "*Required features: `\"UI\"`*"]
    pub fn UIContext(&self) -> ::windows_core::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UIContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UIContext>(result__)
        }
    }
}
impl ::core::clone::Clone for UIContentRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UIContentRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIContentRoot {}
impl ::core::fmt::Debug for UIContentRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIContentRoot").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UIContentRoot {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIContentRoot;{1dfcbac6-b36b-5cb9-9bc5-2b7a0eddc378})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
    const IID: ::windows_core::GUID = <IUIContentRoot as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UIContentRoot {
    const NAME: &'static str = "Windows.UI.UIContentRoot";
}
impl ::core::convert::From<UIContentRoot> for ::windows_core::IUnknown {
    fn from(value: UIContentRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows_core::IUnknown {
    fn from(value: &UIContentRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UIContentRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UIContentRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UIContentRoot> for ::windows_core::IInspectable {
    fn from(value: UIContentRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContentRoot> for ::windows_core::IInspectable {
    fn from(value: &UIContentRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UIContentRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UIContentRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContentRoot {}
unsafe impl ::core::marker::Sync for UIContentRoot {}
#[doc = "*Required features: `\"UI\"`*"]
#[repr(transparent)]
pub struct UIContext(::windows_core::IUnknown);
impl UIContext {}
impl ::core::clone::Clone for UIContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UIContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIContext {}
impl ::core::fmt::Debug for UIContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UIContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.UIContext;{bb5cfacd-5bd8-59d0-a59e-1c17a4d6d243})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UIContext {
    type Vtable = IUIContext_Vtbl;
    const IID: ::windows_core::GUID = <IUIContext as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UIContext {
    const NAME: &'static str = "Windows.UI.UIContext";
}
impl ::core::convert::From<UIContext> for ::windows_core::IUnknown {
    fn from(value: UIContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContext> for ::windows_core::IUnknown {
    fn from(value: &UIContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UIContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UIContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UIContext> for ::windows_core::IInspectable {
    fn from(value: UIContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIContext> for ::windows_core::IInspectable {
    fn from(value: &UIContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UIContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UIContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIContext {}
unsafe impl ::core::marker::Sync for UIContext {}
#[repr(C)]
#[doc = "*Required features: `\"UI\"`*"]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows_core::Abi for WindowId {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for WindowId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WindowId>()) == 0 }
    }
}
impl ::core::cmp::Eq for WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
