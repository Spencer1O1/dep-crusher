use std::hash::Hash;

use crate::crush;
use crate::result::Result;

pub trait Node: Sized {
    type Id: Eq + Hash + Copy;
    type Value: Sized;

    fn get_value(&self) -> Self::Value;
    fn get_id(&self) -> Self::Id;
    fn get_next(&self) -> &Option<Vec<Self>>;
    fn crush(&self) -> Result<Self::Id, Self::Value> {
        crush::<Self>(&self)
    }
}
