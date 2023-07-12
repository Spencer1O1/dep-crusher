use crate::dep_node::Node;

pub type Result<N> = std::result::Result<Vec<N>, Error<N>>;

#[derive(PartialEq, Debug)]
pub enum Error<N: Node> {
    DependencyLoop(Vec<N>),
    Unknown,
}
