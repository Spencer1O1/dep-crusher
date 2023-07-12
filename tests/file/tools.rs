use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use regex::Regex;
use same_file::Handle;

pub fn get_id_from_path(p: PathBuf) -> u64 {
    let handle: Handle = Handle::from_path(p).expect("Couldn't get file handle from path");
    calculate_file_hash(&handle)
}
fn calculate_file_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn get_imports(path: &PathBuf, name: &PathBuf) -> Option<Vec<(PathBuf, PathBuf)>> {
    let contents = fs::read_to_string(path.join(name)).expect("Failed to get file contents...");
    let re = Regex::new(r#"import\s+"(.*)""#).unwrap();

    let mut imports: Vec<(PathBuf, PathBuf)> = Vec::new();

    for (_, [new_rel_path]) in re
        .captures_iter(&contents)
        .map(|c: regex::Captures<'_>| c.extract())
    {
        let (p, n) = match new_rel_path.rsplit_once('/') {
            Some((p, n)) => (p, n),
            None => ("", new_rel_path),
        };

        imports.push((path.join(p), PathBuf::from(n)));
    }

    if imports.is_empty() {
        return None;
    }
    Some(imports)
}
