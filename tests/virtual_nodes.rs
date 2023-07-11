use dep_crusher::DepNode;
use std::collections::HashMap;

struct Node<'a> {
    id: u64,
    value: u64,
    next: Option<Vec<&'a Self>>,
}

impl Node<'_> {
    fn new(id: u64, value: u64, next: Option<Vec<&Self>>) -> Node {
        Node { id, value, next }
    }
}

impl DepNode for Node<'_> {
    type Value = u64;

    fn get_value(&self) -> Self::Value {
        self.value
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_next(&self) -> &Option<Vec<&Self>> {
        &self.next
    }
}

#[test]
fn virtual_nodes() {
    let mut unsorted: HashMap<u64, &Node> = HashMap::new();

    let mut n0: Node<'_> = Node::new(0, 0, None);
    let mut n1: Node<'_> = Node::new(1, 1, None);
    let mut n2: Node<'_> = Node::new(2, 2, None);
    let mut n3: Node<'_> = Node::new(3, 3, None);
    let mut n4: Node<'_> = Node::new(4, 4, None);
    let mut n5: Node<'_> = Node::new(5, 5, None);
    let mut n6: Node<'_> = Node::new(6, 6, None);
    let mut n7: Node<'_> = Node::new(7, 7, None);
    let mut n8: Node<'_> = Node::new(8, 8, None);
    let mut n9: Node<'_> = Node::new(9, 9, None);
    let mut n10: Node<'_> = Node::new(10, 10, None);
    let mut n11: Node<'_> = Node::new(11, 11, None);
    let mut n12: Node<'_> = Node::new(12, 12, None);
    let mut n13: Node<'_> = Node::new(13, 13, None);
    let mut n14: Node<'_> = Node::new(14, 14, None);

    unsorted.insert(n0.id, &n0);
    unsorted.insert(n1.id, &n1);
    unsorted.insert(n2.id, &n2);
    unsorted.insert(n3.id, &n3);
    unsorted.insert(n3.id, &n3);
    unsorted.insert(n5.id, &n5);
    unsorted.insert(n6.id, &n6);
    unsorted.insert(n7.id, &n7);
    unsorted.insert(n8.id, &n8);
    unsorted.insert(n9.id, &n9);
    unsorted.insert(n10.id, &n10);
    unsorted.insert(n11.id, &n11);
    unsorted.insert(n12.id, &n12);
    unsorted.insert(n13.id, &n13);
    unsorted.insert(n13.id, &n13);

    // Create a fake environment with everything linked
    n10.next = None;
    n13.next = None;
    n11.next = Some(vec![&n13]);
    n5.next = Some(vec![&n10, &n11]);
    n6.next = Some(vec![&n11]);
    n1.next = Some(vec![&n5, &n6]);
    n7.next = None;
    n14.next = None;
    n12.next = Some(vec![&n14]);
    n8.next = Some(vec![&n12]);
    n2.next = Some(vec![&n7, &n8]);
    n3.next = Some(vec![&n12]);
    n9.next = None;
    n4.next = Some(vec![&n9]);
    n0.next = Some(vec![&n1, &n2, &n3, &n4]);

    assert_eq!(
        vec![10, 13, 11, 5, 6, 1, 7, 14, 12, 8, 2, 3, 9, 4, 0],
        n0.dep_crush()
    )
}
