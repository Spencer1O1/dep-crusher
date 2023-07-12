use dep_crusher::dep_node::Node as DepNode;

#[derive(Debug, Clone)]
struct Node {
    id: u64,
}

impl Node {
    fn new(id: u64) -> Node {
        Node { id }
    }
}

impl DepNode for Node {
    type Id = u64;

    fn get_id(&self) -> Self::Id {
        self.id
    }

    fn get_next(&self) -> Option<Vec<Self>> {
        match self.id {
            0 => Some(vec![Node::new(1), Node::new(2), Node::new(3), Node::new(4)]),
            1 => Some(vec![Node::new(5), Node::new(6)]),
            2 => Some(vec![Node::new(7), Node::new(8)]),
            3 => Some(vec![Node::new(12)]),
            4 => Some(vec![Node::new(9)]),
            5 => Some(vec![Node::new(10), Node::new(11)]),
            6 => Some(vec![Node::new(11)]),
            7 => None,
            8 => Some(vec![Node::new(12)]),
            9 => None,
            10 => None,
            11 => Some(vec![Node::new(13)]),
            12 => Some(vec![Node::new(14)]),
            13 => None,
            14 => None,
            _ => None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// impl Eq for Node {}

#[test]
fn basic_graph() {
    // let mut unsorted: HashMap<u64, &Node> = HashMap::new();

    let n0 = Node::new(0);
    let n1 = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    let n4 = Node::new(4);
    let n5 = Node::new(5);
    let n6 = Node::new(6);
    let n7 = Node::new(7);
    let n8 = Node::new(8);
    let n9 = Node::new(9);
    let n10 = Node::new(10);
    let n11 = Node::new(11);
    let n12 = Node::new(12);
    let n13 = Node::new(13);
    let n14 = Node::new(14);

    assert_eq!(
        Ok(vec![
            n10,
            n13,
            n11,
            n5,
            n6,
            n1,
            n7,
            n14,
            n12,
            n8,
            n2,
            n3,
            n9,
            n4,
            n0.clone(),
        ]),
        n0.crush()
    )
}

// #[test]
// fn loop_error() {
//     let mut unsorted: HashMap<u64, &Node> = HashMap::new();

//     let mut n0 = Node::new(0, 0, None);
//     let mut n1 = Node::new(1, 1, None);
//     let mut n2 = Node::new(2, 2, None);
//     let mut n3 = Node::new(3, 3, None);
//     let mut n4 = Node::new(4, 4, None);
//     let mut n5 = Node::new(5, 5, None);
//     let mut n6 = Node::new(6, 6, None);
//     let mut n7 = Node::new(7, 7, None);
//     let mut n8 = Node::new(8, 8, None);
//     let mut n9 = Node::new(9, 9, None);
//     let mut n10 = Node::new(10, 10, None);
//     let mut n11 = Node::new(11, 11, None);
//     let mut n12 = Node::new(12, 12, None);
//     let mut n13 = Node::new(13, 13, None);
//     let mut n14 = Node::new(14, 14, None);

//     unsorted.insert(n0.id, &n0);
//     unsorted.insert(n1.id, &n1);
//     unsorted.insert(n2.id, &n2);
//     unsorted.insert(n3.id, &n3);
//     unsorted.insert(n3.id, &n3);
//     unsorted.insert(n5.id, &n5);
//     unsorted.insert(n6.id, &n6);
//     unsorted.insert(n7.id, &n7);
//     unsorted.insert(n8.id, &n8);
//     unsorted.insert(n9.id, &n9);
//     unsorted.insert(n10.id, &n10);
//     unsorted.insert(n11.id, &n11);
//     unsorted.insert(n12.id, &n12);
//     unsorted.insert(n13.id, &n13);
//     unsorted.insert(n13.id, &n13);

//     // Create a fake environment with everything linked
//     n0.next = Some(vec![n1.clone(), n2.clone(), n3.clone(), n4.clone()]);
//     n1.next = Some(vec![n5.clone(), n6.clone()]);
//     n2.next = Some(vec![n7.clone(), n8.clone()]);
//     n3.next = Some(vec![n12.clone()]);
//     n4.next = Some(vec![n9.clone()]);
//     n5.next = Some(vec![n10.clone(), n11.clone()]);
//     n6.next = Some(vec![n11.clone()]);
//     n7.next = None;
//     n8.next = Some(vec![n12.clone()]);
//     n9.next = None;
//     n10.next = None;
//     n11.next = Some(vec![n13.clone()]);
//     n12.next = Some(vec![n14.clone(), n2.clone()]);
//     n13.next = None;
//     n14.next = None;

//     assert_eq!(
//         Err(dep_crusher::result::Error::DependencyLoop(vec![
//             n12, n8, n2
//         ])),
//         n0.crush()
//     )
// }
