use windows::*;

#[test]
fn implement_escape() {}

#[::windows_core::implement(UI::Xaml::Data::ICustomPropertyProvider)]
struct BookShu {}

#[allow(non_snake_case)]
impl UI::Xaml::Data::ICustomPropertyProvider_Impl for BookShu {
    fn Type(&self) -> ::windows_core::Result<UI::Xaml::Interop::TypeName> {
        panic!();
    }

    fn GetCustomProperty(&self, _name: &::windows_core::HSTRING) -> ::windows_core::Result<UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    fn GetIndexedProperty(&self, _name: &::windows_core::HSTRING, _type: &UI::Xaml::Interop::TypeName) -> ::windows_core::Result<UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    fn GetStringRepresentation(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        panic!();
    }
}
