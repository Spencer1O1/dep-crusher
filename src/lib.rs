use std::collections::HashMap;

pub(crate) mod private {
    pub enum Local {}
    pub trait IsLocal {}
    impl IsLocal for Local {}
}

pub type DepNodeId = u64;
pub trait DepNode: Sized {
    type Value: Sized;

    fn get_value(&self) -> Self::Value;
    fn get_id(&self) -> DepNodeId;
    fn get_next(&self) -> &Option<Vec<&Self>>;

    fn dep_crush(&self) -> Result<Vec<Self::Value>, Option<String>> {
        let mut visited: HashMap<DepNodeId, bool> = HashMap::new();
        let mut out: Vec<Self::Value> = Vec::new();
        let mut loop_at_ids: Vec<DepNodeId> = Vec::new();

        match DepNode::visit_node::<private::Local>(self, &mut visited, &mut out, &mut loop_at_ids)
        {
            Ok(_) => Ok(out),
            Err(e) => Err(e),
        }
    }

    #[doc(hidden)]
    fn visit_node<L: private::IsLocal>(
        node: &Self,
        visited: &mut HashMap<DepNodeId, bool>,
        out: &mut Vec<Self::Value>,
        loop_at_ids: &mut Vec<DepNodeId>,
    ) -> Result<(), Option<String>> {
        let id = node.get_id();

        if let Some(first_of_loop) = loop_at_ids.get(0) {
            if &id == first_of_loop {
                return Err(Some(format!(
                    "ERROR: Dependency loop found! No circular dependencies allowed. {:?}",
                    loop_at_ids
                )));
            }
            loop_at_ids.push(id);
            return Err(None);
        }

        if let Some(&added) = visited.get(&id) {
            if added {
                return Ok(());
            } else {
                loop_at_ids.push(id);
                return Err(None);
            }
        }

        visited.insert(id, false);

        if let Some(next) = node.get_next() {
            for &n in next {
                let _ = DepNode::visit_node::<private::Local>(n, visited, out, loop_at_ids);
            }
        }

        visited.insert(id, true);
        out.push(node.get_value());
        Ok(())
    }
}
