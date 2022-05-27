use windows::ApplicationModel::Contacts::KnownContactField;

#[test]
fn test() -> windows_core::Result<()> {
    assert_eq!(KnownContactField::Email()?, "email");

    Ok(())
}
