use windows::{Win32::System::Com::*};

#[cfg(target_pointer_width = "32")]
#[test]
fn size() {
    assert_eq!(std::mem::size_of::<VARIANT>(), 16);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn size() {
    assert_eq!(std::mem::size_of::<VARIANT>(), 24);
}
