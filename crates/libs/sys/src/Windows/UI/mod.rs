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
pub type ColorHelper = *mut ::core::ffi::c_void;
pub type Colors = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IColorHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IColorHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 423427047, data2: 26055, data3: 17728, data4: [173, 8, 98, 131, 186, 118, 135, 154] };
}
#[repr(C)]
pub struct IColorHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromArgb: unsafe extern "system" fn(this: *mut *mut Self, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2231688170, data2: 64362, data3: 16708, data4: [166, 194, 51, 73, 156, 146, 132, 245] };
}
#[repr(C)]
pub struct IColorHelperStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToDisplayName: unsafe extern "system" fn(this: *mut *mut Self, color: Color, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorHelperStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 618245890, data2: 28336, data3: 19348, data4: [133, 92, 252, 240, 129, 141, 154, 22] };
}
#[repr(C)]
pub struct IColors {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IColors {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2609681190, data2: 19622, data3: 19685, data4: [137, 148, 158, 255, 101, 202, 189, 204] };
}
#[repr(C)]
pub struct IColorsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AliceBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Aqua: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Azure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Beige: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Bisque: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Black: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Blue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Brown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Coral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Crimson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Cyan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DimGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Gold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Gray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Green: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub HotPink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Indigo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Ivory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Khaki: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Lavender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightPink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Lime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Linen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Magenta: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Maroon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MintCream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Navy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub OldLace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Olive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Orange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Orchid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Peru: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Pink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Plum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Purple: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Red: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Salmon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Sienna: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Silver: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Snow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Tan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Teal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Thistle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Tomato: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Transparent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Violet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Wheat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub White: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub Yellow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Color) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IColorsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488951812, data2: 52390, data3: 17940, data4: [161, 126, 117, 73, 16, 200, 74, 153] };
}
#[repr(C)]
pub struct IUIContentRoot {
    pub base__: ::windows_sys::core::IInspectable,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUIContentRoot {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 503102150, data2: 45931, data3: 23737, data4: [155, 197, 43, 122, 14, 221, 195, 120] };
}
#[repr(C)]
pub struct IUIContext {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IUIContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3143432909, data2: 23512, data3: 22992, data4: [165, 158, 28, 23, 164, 214, 210, 67] };
}
pub type UIContentRoot = *mut ::core::ffi::c_void;
pub type UIContext = *mut ::core::ffi::c_void;
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
