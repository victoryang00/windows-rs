fn main() -> windows_core::Result<()> {
    use windows::UI::Colors;

    let red = Colors::Red()?;
    println!("Red: {:?}", red);

    Ok(())
}
