use metadata::reader::*;
use std::collections::*;

fn main() {
    let files = vec![File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &Reader::new(&files);
    let root = reader.tree("Windows").unwrap();
    let mut crates = BTreeMap::<String, Vec<&str>>::new();
    for tree in root.flatten() {
        let split: Vec<&str> = tree.namespace.split('.').collect();
        if split.get(1) == Some(&"Win32") {
            if let Some(name) = split.get(2) {
                crates.entry(format!("Win32.{}", name)).or_default().push(tree.namespace);
            }
        } else if let Some(name) = split.get(1) {
            crates.entry(name.to_string()).or_default().push(tree.namespace);
        }
    }
    let _ = std::fs::remove_dir_all("crates/generated");
    for (name, namespaces) in crates {
        build(reader, &name, &namespaces, true);
        build(reader, &name, &namespaces, false);
    }
}

fn build(reader: &Reader, root: &str, namespaces: &[&str], sys: bool) {
    let name = if sys { "windows-sys" } else { "windows" };
    let name = format!("{}-{}", name, root.to_lowercase().replace(".", "-"));
    println!("{}", name);
    let mut path = std::path::PathBuf::from("crates/generated");
    path.push(&name);
    std::fs::create_dir_all(&path).unwrap();
    path.push("Cargo.toml");

    std::fs::write(
        &path,
        format!(
            r#"
[package]
name = "{}"
version = "0.22.6"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../.github/readme.md"
rust-version = "1.46"
"#,
            name
        ),
    )
    .unwrap();

    path.pop();
    path.push("src");
    std::fs::create_dir_all(&path).unwrap();
    path.push("lib.rs");
    std::fs::write(&path, "").unwrap();

    path.pop();
    path.pop();

    // loop {
    //     let mut cmd = std::process::Command::new("cargo");
    //     cmd.current_dir(&path);
    //     cmd.arg("publish");
    //     cmd.arg("--allow-dirty");
    //     let output = cmd.output().unwrap();

    //     if output.status.success() {
    //         break;
    //     } else {
    //         std::io::stdout().write_all(&output.stdout).unwrap();
    //         std::io::stderr().write_all(&output.stderr).unwrap();
    //         println!("Waiting to retry...");
    //         std::thread::sleep(std::time::Duration::from_secs(60*6));
    //     }
    // }
}
