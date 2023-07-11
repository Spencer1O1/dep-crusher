use crate::dep_node::Node;
use std::collections::HashMap;

pub type VisitResult<N> = std::result::Result<(), VisitError<N>>;
pub enum VisitError<N: Node> {
    LoopDetected(N::Id),
    LoopPropagate(N::Id, Vec<N::Id>),
    LoopCompleted(Vec<N::Id>),
}

pub fn visit_node<N: Node>(
    node: &N,
    visited: &mut HashMap<N::Id, bool>,
    out: &mut Vec<N::Value>,
) -> VisitResult<N> {
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
                    VisitError::<N>::LoopDetected(i) => {
                        let ids: Vec<N::Id> = vec![id];
                        if id == i {
                            return Err(VisitError::LoopCompleted(ids));
                        }
                        Err(VisitError::LoopPropagate(i, ids))
                    }
                    VisitError::<N>::LoopPropagate(i, mut ids) => {
                        ids.push(id);
                        if id == i {
                            return Err(VisitError::LoopCompleted(ids));
                        }
                        Err(VisitError::LoopPropagate(i, ids))
                    }
                    VisitError::<N>::LoopCompleted(ids) => Err(VisitError::LoopCompleted(ids)),
                };
            };
        }
    }

    visited.insert(id, true);
    out.push(node.get_value());
    Ok(())
}
