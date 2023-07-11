use std::collections::HashMap;

pub(crate) mod private {
    pub enum Local {}
    pub trait IsLocal {}
    impl IsLocal for Local {}
}

pub trait DepNode: Sized {
    type Value: Sized;

    fn get_value(&self) -> Self::Value;
    fn get_id(&self) -> u64;
    fn get_next(&self) -> &Option<Vec<&Self>>;

    fn dep_crush(&self) -> Vec<Self::Value> {
        let mut visited: HashMap<u64, bool> = HashMap::new();
        let mut out: Vec<Self::Value> = Vec::new();

        DepNode::visit_node::<private::Local>(self, &mut visited, &mut out);

        out
    }

    #[doc(hidden)]
    fn visit_node<L: private::IsLocal>(
        node: &Self,
        visited: &mut HashMap<u64, bool>,
        out: &mut Vec<Self::Value>,
    ) -> Option<Result<(), ()>> {
        let id = node.get_id();

        if let Some(&added) = visited.get(&id) {
            if added {
                return Some(Ok(()));
            } else {
                return Some(Err(()));
            }
        }

        visited.insert(id, false);

        if let Some(next) = node.get_next() {
            for &n in next {
                DepNode::visit_node::<private::Local>(n, visited, out);
            }
        }

        visited.insert(id, true);
        out.push(node.get_value());
        None
    }
}
