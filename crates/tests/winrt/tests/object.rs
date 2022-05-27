use core::convert::TryInto;
use windows::Foundation::{IStringable, PropertyValue, Uri};

#[test]
fn class() -> windows_core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;

    // All WinRT classes are convertible to windows_core::IInspectable.
    let object: windows_core::IInspectable = uri.into();

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn interface() -> windows_core::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca")?;
    let stringable: IStringable = uri.try_into().unwrap();

    // All WinRT interfaces are convertible to windows_core::IInspectable.
    let object: windows_core::IInspectable = stringable.into();

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.Uri");

    Ok(())
}

#[test]
fn boxing() -> windows_core::Result<()> {
    let object = PropertyValue::CreateString("hello")?;

    assert!(object.GetRuntimeClassName()? == "Windows.Foundation.IReference`1<String>");

    Ok(())
}
