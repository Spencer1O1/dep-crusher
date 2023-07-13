use dep_crusher::{dep_node::Node as DepNode, result::Error};

#[derive(Debug, Clone, Copy)]
enum TestMode {
    Basic,
    Loop,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    id: u64,
    test_mode: TestMode,
}

impl Node {
    fn new(id: u64, test_mode: TestMode) -> Node {
        Node { id, test_mode }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DepNode for Node {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        self.id
    }

    fn get_next(&self) -> Option<Vec<Self>> {
        // Since these are virtual nodes we have to manually "link" the nodes somehow
        // The test_mode property is used to change the linkages between tests.
        // For example, when test_mode is Loop, 12 requires 8 requires 2 requires 12. A loop.
        match self.id {
            0 => Some(vec![
                Node::new(1, self.test_mode),
                Node::new(2, self.test_mode),
                Node::new(3, self.test_mode),
                Node::new(4, self.test_mode),
            ]),
            1 => Some(vec![
                Node::new(5, self.test_mode),
                Node::new(6, self.test_mode),
            ]),
            2 => Some(vec![
                Node::new(7, self.test_mode),
                Node::new(8, self.test_mode),
            ]),
            3 => Some(vec![Node::new(12, self.test_mode)]),
            4 => Some(vec![Node::new(9, self.test_mode)]),
            5 => Some(vec![
                Node::new(10, self.test_mode),
                Node::new(11, self.test_mode),
            ]),
            6 => Some(vec![Node::new(11, self.test_mode)]),
            7 => None,
            8 => Some(vec![Node::new(12, self.test_mode)]),
            9 => None,
            10 => None,
            11 => Some(vec![Node::new(13, self.test_mode)]),
            12 => match self.test_mode {
                TestMode::Basic => Some(vec![Node::new(14, self.test_mode)]),
                TestMode::Loop => Some(vec![
                    Node::new(2, self.test_mode),
                    Node::new(14, self.test_mode),
                ]),
            },
            13 => None,
            14 => None,
            _ => None,
        }
    }
}

#[test]
fn basic_graph() {
    let test_mode = TestMode::Basic;

    let mut nodes = Vec::new();

    for i in 0..=14 {
        nodes.push(Node::new(i, test_mode));
    }

    let correct_ordered_nodes: Vec<Node> = [10, 13, 11, 5, 6, 1, 7, 14, 12, 8, 2, 3, 9, 4, 0]
        .iter()
        .map(|&i| *nodes.get(i).unwrap())
        .collect();

    assert_eq!(Ok(correct_ordered_nodes), nodes.get(0).unwrap().crush())
}

#[test]
fn loop_error() {
    let test_mode = TestMode::Loop;

    let mut nodes = Vec::new();

    for i in 0..=14 {
        nodes.push(Node::new(i, test_mode));
    }

    assert_eq!(
        Err(Error::DependencyLoop(vec![
            *nodes.get(12).unwrap(),
            *nodes.get(8).unwrap(),
            *nodes.get(2).unwrap(),
        ])),
        nodes.get(0).unwrap().crush()
    )
}
