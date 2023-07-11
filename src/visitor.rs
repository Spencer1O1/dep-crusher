use crate::dep_node::{Node, NodeId};
use std::collections::HashMap;

pub type VisitResult = std::result::Result<(), VisitError>;
pub enum VisitError {
    LoopDetected(NodeId),
    LoopPropagate(NodeId, Vec<NodeId>),
    LoopCompleted(Vec<NodeId>),
}

pub fn visit_node<N: Node>(
    node: &N,
    visited: &mut HashMap<NodeId, bool>,
    out: &mut Vec<N::Value>,
) -> VisitResult {
    let id = node.get_id();

    if let Some(&added) = visited.get(&id) {
        if added {
            return Ok(());
        } else {
            return Err(VisitError::LoopDetected(id));
        }
    }

    visited.insert(id, false);

    if let Some(next) = node.get_next() {
        for n in next {
            if let Err(e) = visit_node::<N>(n, visited, out) {
                return match e {
                    VisitError::LoopDetected(i) => {
                        let ids: Vec<NodeId> = vec![id];
                        if id == i {
                            return Err(VisitError::LoopCompleted(ids));
                        }
                        Err(VisitError::LoopPropagate(i, ids))
                    }
                    VisitError::LoopPropagate(i, mut ids) => {
                        ids.push(id);
                        if id == i {
                            return Err(VisitError::LoopCompleted(ids));
                        }
                        Err(VisitError::LoopPropagate(i, ids))
                    }
                    VisitError::LoopCompleted(ids) => Err(VisitError::LoopCompleted(ids)),
                };
            };
        }
    }

    visited.insert(id, true);
    out.push(node.get_value());
    Ok(())
}
