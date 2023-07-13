use std::hash::Hash;

use crate::crush;
use crate::result::Result;

pub trait Node: Sized + PartialEq {
    type Id: Eq + Hash;

    fn get_id(&self) -> Self::Id;
    fn get_next(&self) -> Option<Vec<Self>>;
    fn crush(self) -> Result<Self> {
        crush::<Self>(self)
    }
}
