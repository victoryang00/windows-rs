use std::collections::*;
use metadata::reader::*;
use std::fmt::Write;

fn main() {
    let files = vec![File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &Reader::new(&files);
    let root = reader.tree("Windows").unwrap();
    let mut map = BTreeMap::<&str, BTreeSet<&str>>::new();
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
    let mut cycles = BTreeSet::<String>::new();
    for namespace in map.keys() {
        let mut path = Vec::new();
        cycle(&map, namespace, &mut path, &mut cycles);
    }
    for cycle in cycles {
        println!("{}", cycle);
    }
}

fn cycle<'a>(map: &BTreeMap<&'a str, BTreeSet<&'a str>>, next: &'a str, path: &mut Vec<&'a str>, cycles: &mut BTreeSet<String>) -> usize {
    path.push(next);
    for dependency in &map[path[path.len() - 1]] {
        if let Some(pos) = path.iter().position(|path| path == dependency) {
            path.push(dependency);
            cycles.insert(format_cycle(path));
            path.pop();
            return path.len() - pos;
        }
        let pop = cycle(map, dependency, path, cycles);
        if pop > 0 {
            return pop - 1;
        }
    }
    path.pop();
    0
}

fn format_cycle(path: &[&str]) -> String {
    let mut result = String::new();
    for ns in path {
        write!(result, "{} -> ", ns).unwrap();
    }
    result.truncate(result.len() - 4);
    result.push('\n');
    result
}
