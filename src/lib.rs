use std::collections::HashMap;

use dep_node::Node;
use result::{Error, Result};
use visitor::{visit_node, VisitError};

pub mod dep_node;
pub mod result;
mod visitor;

pub fn crush<N: Node>(node: N) -> Result<N> {
    let mut out: Vec<N> = Vec::new();

    match visit_node::<N>(node, &mut HashMap::new(), &mut out) {
        Ok(()) => Ok(out),
        Err(VisitError::<N>::LoopCompleted(loop_data)) => Err(Error::DependencyLoop(loop_data)),
        Err(_) => Err(Error::Unknown),
    }
}
