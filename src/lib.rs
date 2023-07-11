use std::collections::HashMap;

use dep_node::{Node, NodeId};
use result::{Error, Result};
use visitor::{visit_node, VisitError};

pub mod dep_node;
pub mod result;
mod visitor;

pub fn crush<N: Node>(node: &N) -> Result<NodeId, N::Value> {
    let mut out: Vec<N::Value> = Vec::new();

    match visit_node::<N>(node, &mut HashMap::new(), &mut out) {
        Ok(()) => Ok(out),
        Err(VisitError::LoopCompleted(ids)) => Err(Error::DependencyLoop(ids)),
        Err(_) => Err(Error::Unknown),
    }
}
