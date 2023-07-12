use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use dep_crusher::dep_node::Node;
use regex::Regex;
use same_file::Handle;

pub fn get_id_from_path<P: AsRef<Path>>(p: P) -> u64 {
    let handle: Handle = Handle::from_path(p).expect("Couldn't get file handle from path");
    calculate_file_hash(&handle)
}
fn calculate_file_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct FileNode {
    path: PathBuf,
    name: PathBuf,
}

impl Node for FileNode {
    type Id = u64;
    type Value = String;

    fn get_value(&self) -> Self::Value {
        Path::join(&self.path, &self.name)
            .to_str()
            .expect("Couldn't convert path to string...")
            .to_string()
    }

    fn get_id(&self) -> Self::Id {
        get_id_from_path(Path::join(&self.path, &self.name))
    }

    fn get_next(&self) -> Option<Vec<Self>> {
        let contents = fs::read_to_string(Path::join(&self.path, &self.name))
            .expect("Failed to get file contents...");
        let re = Regex::new(r#"import\s+"(.*)""#).unwrap();

        let mut next: Vec<Self> = Vec::new();

        for (_, [full_path]) in re
            .captures_iter(&contents)
            .map(|c: regex::Captures<'_>| c.extract())
        {
            let (path, name) = match full_path.rsplit_once('/') {
                Some((path, name)) => (path, name),
                None => ("", full_path),
            };

            next.push(FileNode {
                path: Path::join(&self.path, PathBuf::from(path)),
                name: PathBuf::from(name),
            });
        }

        if next.is_empty() {
            return None;
        }
        Some(next)
    }
}

#[test]
fn basic_file_graph() {
    let index: FileNode = FileNode {
        path: PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_files")),
        name: PathBuf::from("0"),
    };

    assert_eq!(
        Ok(vec![
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./aa\\.\\10".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./aa\\../../b\\./ba\\13".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./aa\\../../b\\11".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./aa\\5".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\.\\6".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\1".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\..\\7".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./ab\\.\\../..\\14".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./ab\\.\\12".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\./ab\\8".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./a\\2".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\.\\3".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./b\\../a\\9".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\./b\\4".to_owned(),
            "C:\\Dev\\dep-crusher/tests/test_files\\0".to_owned()
        ]),
        dep_crusher::crush(&index)
    );
}
