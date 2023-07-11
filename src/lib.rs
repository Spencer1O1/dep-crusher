use std::collections::HashMap;

pub type DepNodeId = u64;
pub trait DepNode: Sized {
    type Value: Sized;

    fn get_value(&self) -> Self::Value;
    fn get_id(&self) -> DepNodeId;
    fn get_next(&self) -> &Option<Vec<Self>>;

    fn dep_crush(&self) -> Result<Vec<Self::Value>, Option<String>> {
        let mut out: Vec<Self::Value> = Vec::new();

        match visit_node::<Self>(self, &mut HashMap::new(), &mut out) {
            Ok(()) => Ok(out),
            Err(VisitError::LoopCompleted(ids)) => {
                Err(Some(format!("A dependency loop was found: {:?}", ids)))
            }
            Err(_) => Err(Some("An error occured while visiting nodes...".to_owned())),
        }
    }
}

type VisitResult = Result<(), VisitError>;
enum VisitError {
    LoopDetected(DepNodeId),
    LoopPropagate(DepNodeId, Vec<DepNodeId>),
    LoopCompleted(Vec<DepNodeId>),
}

fn visit_node<N: DepNode>(
    node: &N,
    visited: &mut HashMap<DepNodeId, bool>,
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
                        let ids: Vec<DepNodeId> = vec![id];
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
