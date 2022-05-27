use metadata::reader::*;
use rayon::prelude::*;
use std::io::Write;

fn main() {
    let files = vec![File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &Reader::new(&files);
    let _ = std::fs::remove_dir_all("crates/generated");
    let root = reader.tree("Windows").unwrap();
    let mut trees = vec![];
    for (name, tree) in &root.nested {
        if name == &"Win32" {
            for tree in tree.nested.values() {
                trees.push(tree);
            }
        } else {
            trees.push(tree);
        }
    }
    trees
    .par_iter()
    .for_each(|tree| build(reader, tree));
   // build(reader, &root.nested["Foundation"]);
}

fn build(reader: &Reader, tree: &Tree) {
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

    let mut file = std::fs::File::create(&path).unwrap();

    write!(
        file,
        r#"
[package]
name = "{}"
version = "0.37.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../.github/readme.md"
rust-version = "1.46"
"#,
        name
    )
    .unwrap();

    if sys {
        file.write_all(
            r#"
[dependencies]
windows-sys-core = { path = "../../libs/windows-sys-core",  version = "0.37.0" }
"#
            .as_bytes(),
        )
        .unwrap();
    } else {
        file.write_all(
            r#"
[dependencies]
windows-core = { path = "../../libs/windows-core",  version = "0.37.0" }
"#
            .as_bytes(),
        )
        .unwrap();
    }

    write!(
        file,
        r#"
[features]
default = []
deprecated = []
"#,
    )
    .unwrap();

    file.write_all(imp::gen_features(tree).as_bytes()).unwrap();

    path.pop();
    path.push("src");
    build_tree(reader, "", tree, &path, sys, true);
}

fn build_tree(reader: &Reader, name: &str, tree: &Tree, path: &std::path::Path, sys: bool, root: bool) {
    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.min_xaml = true;
    gen.cfg = true;
    gen.sys = sys;

    let mut path = std::path::PathBuf::from(path);
    if root {
        let mut lib = r#"
        #![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
            "#
        .to_string();
        if sys {
            lib.push_str("#![no_std]");
        }
        lib.push_str(&bindgen::namespace(&gen, tree));
        imp::write_fmt(&path.join("lib.rs"), lib);
    } else {
        path.push(name);
        imp::write_fmt(&path.join("mod.rs"), bindgen::namespace(&gen, tree))
    }
    for (name, tree) in &tree.nested {
        build_tree(reader, name, tree, &path, sys, false);
    }
}
