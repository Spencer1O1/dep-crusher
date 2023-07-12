use crate::dep_node::Node;
use std::collections::HashMap;

pub type VisitResult<N> = std::result::Result<(), VisitError<N>>;
pub enum VisitError<N: Node> {
    LoopDetected((N::Id, N::Value)),
    LoopPropagate((N::Id, N::Value), Vec<(N::Id, N::Value)>),
    LoopCompleted(Vec<(N::Id, N::Value)>),
}

pub fn visit_node<N: Node>(
    node: &N,
    visited: &mut HashMap<N::Id, bool>,
    out: &mut Vec<N::Value>,
) -> VisitResult<N> {
    let id = node.get_id();
    let value = node.get_value();

    if let Some(&added) = visited.get(&id) {
        if added {
            return Ok(());
        } else {
            return Err(VisitError::LoopDetected((id, value)));
        }
    }

    visited.insert(id, false);

    if let Some(next) = node.get_next() {
        for n in next {
            if let Err(e) = visit_node::<N>(&n, visited, out) {
                return match e {
                    VisitError::<N>::LoopDetected(e) => {
                        let es: Vec<(N::Id, N::Value)> = vec![(id, value)];
                        if id == e.0 {
                            return Err(VisitError::LoopCompleted(es));
                        }
                        Err(VisitError::LoopPropagate(e, es))
                    }
                    VisitError::<N>::LoopPropagate(e, mut es) => {
                        es.push((id, value));
                        if id == e.0 {
                            return Err(VisitError::LoopCompleted(es));
                        }
                        Err(VisitError::LoopPropagate(e, es))
                    }
                    VisitError::<N>::LoopCompleted(es) => Err(VisitError::LoopCompleted(es)),
                };
            };
        }
    }

    visited.insert(id, true);
    out.push(value);
    Ok(())
}
