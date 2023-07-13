use crate::dep_node::Node;
use std::collections::HashMap;

pub type VisitResult<N> = std::result::Result<(), VisitError<N>>;
pub enum InternalError<N: Node> {
    LoopDetected(N),
    LoopPropagate(N, Vec<N>),
}
pub enum VisitError<N: Node> {
    Loop(Vec<N>),
    Internal(InternalError<N>),
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
            return Err(VisitError::Internal(InternalError::LoopDetected(node)));
        }
    }

    visited.insert(node.get_id(), false);

    let next = node.get_next();
    for n in next {
        if let Err(e) = visit_node::<N>(n, visited, out) {
            return match e {
                VisitError::Internal(InternalError::LoopDetected(fail_node)) => {
                    if node == fail_node {
                        Err(VisitError::Loop(vec![node]))
                    } else {
                        Err(VisitError::Internal(InternalError::LoopPropagate(
                            fail_node,
                            vec![node],
                        )))
                    }
                }
                VisitError::Internal(InternalError::LoopPropagate(fail_node, mut loop_nodes)) => {
                    if node == fail_node {
                        loop_nodes.push(node);
                        Err(VisitError::Loop(loop_nodes))
                    } else {
                        loop_nodes.push(node);
                        Err(VisitError::Internal(InternalError::LoopPropagate(
                            fail_node, loop_nodes,
                        )))
                    }
                }
                VisitError::Loop(loop_data) => Err(VisitError::Loop(loop_data)),
            };
        };
    }

    visited.insert(node.get_id(), true);
    out.push(node);
    Ok(())
}
