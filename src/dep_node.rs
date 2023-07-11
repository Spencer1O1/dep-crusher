use crate::crush;
use crate::result::Result;

pub type NodeId = u64;
pub trait Node: Sized {
    type Value: Sized;

    fn get_value(&self) -> Self::Value;
    fn get_id(&self) -> NodeId;
    fn get_next(&self) -> &Option<Vec<Self>>;
    fn crush(&self) -> Result<NodeId, Self::Value> {
        crush::<Self>(&self)
    }
}
