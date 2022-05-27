use windows_core::Interface;
use windows::Foundation::IStringable;

#[test]
fn interface() -> windows_core::Result<()> {
    assert_eq!(IStringable::IID, windows_core::GUID::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3"));

    // TODO: Find an example where the default constructor is not exclusive.
    Ok(())
}
