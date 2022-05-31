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
    let win32 = tree.namespace.starts_with("Windows.Win32");
    let mut name = if win32 {
        tree.namespace[8..].to_lowercase().replace(".", "-")
    } else {
        format!("winrt-{}", tree.namespace[8..].to_lowercase().replace(".", "-"))
    };
    if sys {
        name.push_str("-sys");
    }
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
name = "{name}"
version = "0.37.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../.github/readme.md"
rust-version = "1.46"

[dependencies]
"#
    )
    .unwrap();

    let foundation = if win32 {
        if sys {
            "win32-foundation-sys"    
        } else {
            "win32-foundation"
        }
    } else {
        if sys {
            "winrt-foundation-sys"    
        } else {
            "winrt-foundation"
        }
    };

    let core = if sys {
        "windows-core-sys"
    } else {
        "windows-core"
    };

    writeln!(file, r#"{core} = {{ path = "../../libs/{core}",  version = "0.37.0" }}"#).unwrap();

    if name != foundation {
        writeln!(file, r#"{foundation} = {{ path = "../{foundation}",  version = "0.37.0", optional = true }}"#).unwrap();
    }

    write!(
        file,
        r#"
[features]
deprecated = []
"#
    )
    .unwrap();

    if name != foundation {
        writeln!(file, r#"default = ["{foundation}"]"#).unwrap();
    }

    file.write_all(imp::gen_features(tree).as_bytes()).unwrap();

    path.pop();
    path.push("src");
    build_tree(reader, "", tree, &path, sys, tree.namespace);

    //path.pop();
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
    //         std::thread::sleep(std::time::Duration::from_secs(60*11));
    //     }
    // }
}

fn build_tree(reader: &Reader, name: &str, tree: &Tree, path: &std::path::Path, sys: bool, root: &str) {
    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.root = root;
    gen.min_xaml = true;
    gen.cfg = true;
    gen.sys = sys;

    let mut path = std::path::PathBuf::from(path);
    if root == tree.namespace {
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
        build_tree(reader, name, tree, &path, sys, root);
    }
}
