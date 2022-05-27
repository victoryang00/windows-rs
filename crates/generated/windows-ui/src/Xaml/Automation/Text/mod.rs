#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextPatternRangeEndpoint(pub i32);
impl TextPatternRangeEndpoint {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
}
impl ::core::marker::Copy for TextPatternRangeEndpoint {}
impl ::core::clone::Clone for TextPatternRangeEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextPatternRangeEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TextPatternRangeEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextPatternRangeEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPatternRangeEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TextPatternRangeEndpoint {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextPatternRangeEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextUnit(pub i32);
impl TextUnit {
    pub const Character: Self = Self(0i32);
    pub const Format: Self = Self(1i32);
    pub const Word: Self = Self(2i32);
    pub const Line: Self = Self(3i32);
    pub const Paragraph: Self = Self(4i32);
    pub const Page: Self = Self(5i32);
    pub const Document: Self = Self(6i32);
}
impl ::core::marker::Copy for TextUnit {}
impl ::core::clone::Clone for TextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TextUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TextUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
