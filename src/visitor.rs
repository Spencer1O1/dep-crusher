use crate::dep_node::Node;
use std::collections::HashMap;

pub type VisitResult<N> = std::result::Result<(), VisitError<N>>;
pub enum VisitError<N: Node> {
    LoopDetected(N),
    LoopPropagate(N, Vec<N>),
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
                            Err(VisitError::LoopCompleted(vec![node]))
                        } else {
                            Err(VisitError::LoopPropagate(fail_node, vec![node]))
                        }
                    }
                    VisitError::<N>::LoopPropagate(fail_node, mut loop_nodes) => {
                        if node == fail_node {
                            loop_nodes.push(node);
                            Err(VisitError::LoopCompleted(loop_nodes))
                        } else {
                            loop_nodes.push(node);
                            Err(VisitError::LoopPropagate(fail_node, loop_nodes))
                        }
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
