use std::io::prelude::*;
use metadata::reader::*;

pub fn write_fmt(path: &std::path::Path, mut tokens: String) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("rustfmt failed {}", path.display());
    }

    std::fs::write(&path, tokens).unwrap();
}

pub fn gen_features(tree:&Tree) -> String {
    fn gen(root:&Tree, tree: &Tree) -> String {
        let mut result = String::new();
        if root.namespace != tree.namespace {
            let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

            if let Some(pos) = feature.rfind('_') {
                let dependency = &feature[..pos];
    
                result.push_str(&format!("{} = [\"{}\"]\n", feature, dependency));
            } else {
                result.push_str(&format!("{} = []\n", feature));
            }
        }
        for tree in tree.nested.values() {
            result.push_str(&gen(root, tree));
        }
        result
    }
    gen(tree, tree)
}
