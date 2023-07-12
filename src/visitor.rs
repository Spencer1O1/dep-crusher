use crate::dep_node::Node;
use std::collections::HashMap;

pub type VisitResult<N> = std::result::Result<(), VisitError<N>>;
pub enum VisitError<N: Node> {
    LoopDetected(N),
    LoopPropagate(Vec<N>),
    LoopCompleted(Vec<N>),
}

pub fn visit_node<N: Node>(
    node: N,
    visited: &mut HashMap<N::Id, bool>,
    out: &mut Vec<N>,
) -> VisitResult<N> {
    if let Some(&added) = visited.get(&node.get_id()) {
        if added {
            return Ok(());
        } else {
            return Err(VisitError::LoopDetected(node));
        }
    }

    visited.insert(node.get_id(), false);

    if let Some(next) = node.get_next() {
        for n in next {
            if let Err(e) = visit_node::<N>(n, visited, out) {
                return match e {
                    VisitError::<N>::LoopDetected(fail_node) => {
                        if node == fail_node {
                            return Err(VisitError::LoopCompleted(vec![fail_node]));
                        }
                        Err(VisitError::LoopPropagate(vec![fail_node]))
                    }
                    VisitError::<N>::LoopPropagate(mut loop_nodes) => {
                        if Some(&node) == loop_nodes.first() {
                            loop_nodes.push(node);
                            return Err(VisitError::LoopCompleted(loop_nodes));
                        }
                        loop_nodes.push(node);
                        Err(VisitError::LoopPropagate(loop_nodes))
                    }
                    VisitError::<N>::LoopCompleted(loop_data) => {
                        Err(VisitError::LoopCompleted(loop_data))
                    }
                };
            };
        }
    }

    visited.insert(node.get_id(), true);
    out.push(node);
    Ok(())
}
