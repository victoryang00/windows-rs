use metadata::reader::*;
use std::collections::*;

fn main() {
    let files = vec![File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &Reader::new(&files);
    let _ = std::fs::remove_dir_all("crates/generated");
    let root = reader.tree("Windows").unwrap();
    for (name, tree) in root.nested {
        if name == "Win32" {
            for tree in tree.nested.values() {
                build(reader, &tree);
            }
        } else {
            build(reader, &tree);
        }
    }
}

fn build(reader:&Reader, tree:&Tree) {
    build_kind(reader, tree, true);
    build_kind(reader, tree, false);
}

fn build_kind(reader: &Reader, tree: &Tree, sys: bool) {
    let name = if sys { "windows-sys" } else { "windows" };
    let name = format!("{}-{}", name, tree.namespace.strip_prefix("Windows.").unwrap().to_lowercase().replace(".", "-"));
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
