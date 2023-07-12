use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use dep_crusher::dep_node::Node;
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

#[cfg(target_family = "windows")]
#[test]
fn basic_file_graph() {
    let index: FileNode = FileNode {
        path: PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_files")),
        name: PathBuf::from("0"),
    };
    let test_file_str = index
        .path
        .to_str()
        .expect("Failed to get test_files path string");

    // I know all the paths are weird, but I just wanted a quick implementation for this test.
    // In a production use case you'd probably clean up the paths when you combine them, so
    // you wouldn't have stuff like "\\./". Also this test will probably only pass on windows
    // because that's what I used to generate all the paths.
    assert_eq!(
        Ok(vec![
            test_file_str.to_owned() + "\\./a\\./aa\\.\\10",
            test_file_str.to_owned() + "\\./a\\./aa\\../../b\\./ba\\13",
            test_file_str.to_owned() + "\\./a\\./aa\\../../b\\11",
            test_file_str.to_owned() + "\\./a\\./aa\\5",
            test_file_str.to_owned() + "\\./a\\.\\6",
            test_file_str.to_owned() + "\\./a\\1",
            test_file_str.to_owned() + "\\./a\\..\\7",
            test_file_str.to_owned() + "\\./a\\./ab\\.\\../..\\14",
            test_file_str.to_owned() + "\\./a\\./ab\\.\\12",
            test_file_str.to_owned() + "\\./a\\./ab\\8",
            test_file_str.to_owned() + "\\./a\\2",
            test_file_str.to_owned() + "\\.\\3",
            test_file_str.to_owned() + "\\./b\\../a\\9",
            test_file_str.to_owned() + "\\./b\\4",
            test_file_str.to_owned() + "\\0"
        ]),
        dep_crusher::crush(&index)
    );
}
