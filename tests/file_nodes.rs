use std::path::{Path, PathBuf};

use dep_crusher::dep_node::Node;
use file::tools::{get_id_from_path, get_imports};

mod file;

#[derive(Debug)]
struct FileNode {
    path: PathBuf,
    name: PathBuf,
}

impl FileNode {
    fn new(rel_dir: &str, name: &str) -> Self {
        Self {
            path: Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(["tests", "test_files"].iter().collect::<PathBuf>())
                .join(PathBuf::from(rel_dir)),
            name: PathBuf::from(name),
        }
    }
}

impl PartialEq for FileNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Node for FileNode {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        get_id_from_path(self.path.join(&self.name))
    }

    fn get_next(&self) -> Vec<Self> {
        match get_imports(&self.path, &self.name) {
            Some(i) => i
                .iter()
                .map(|x| x.to_owned())
                .map(|x| FileNode {
                    path: x.0,
                    name: x.1,
                })
                .collect::<Vec<FileNode>>(),
            None => Vec::new(),
        }
    }
}

#[test]
fn basic_file_graph() {
    let index = FileNode::new("", "0");

    let ordered = dep_crusher::crush(index);
    assert_eq!(
        Ok(vec![
            FileNode::new("a/aa", "10"),
            FileNode::new("b/ba", "13"),
            FileNode::new("b", "11"),
            FileNode::new("a/aa", "5"),
            FileNode::new("a", "6"),
            FileNode::new("a", "1"),
            FileNode::new("", "7"),
            FileNode::new("", "14"),
            FileNode::new("a/ab", "12"),
            FileNode::new("a/ab", "8"),
            FileNode::new("a", "2"),
            FileNode::new("", "3"),
            FileNode::new("a", "9"),
            FileNode::new("b", "4"),
            FileNode::new("", "0"),
        ]),
        ordered
    );

    if let Ok(mut ord) = ordered {
        // Do something with the ordered list
        for n in &mut ord {
            n.path = n.path.canonicalize().expect("Failed to canonicalize path!");
        }
        println!("{:#?}", ord);
    }
}
