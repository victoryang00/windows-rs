fn main() {
    let files = vec![metadata::reader::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &metadata::reader::Reader::new(&files);
    let root = reader.tree("Windows").expect("`Windows` namespace not found");
    let mut map = std::collections::BTreeMap::<&str, std::collections::BTreeSet<&str>>::new();
    for tree in root.flatten() {
        let set = map.entry(tree.namespace).or_default();
        if let Some(types) = reader.namespace_types(tree.namespace) {
            for def in types {
                for def in reader.type_def_cfg_crate(def, &[]).types.values().flatten() {
                    let name = reader.type_def_type_name(*def);
                    if !name.namespace.is_empty() && name.namespace != tree.namespace {
                        set.insert(name.namespace);
                    }
                }
            }
        }
    }
    for (count, (namespace, types)) in map.iter().enumerate() {
        if !types.is_empty() {
            println!("\n{} {}", count, namespace);
            for name in types {
                println!("    {}", name);
            }
        }
    }
}
