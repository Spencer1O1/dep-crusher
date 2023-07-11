pub type Result<Id, V> = std::result::Result<Vec<V>, Error<Id>>;

#[derive(PartialEq, Debug)]
pub enum Error<Id> {
    DependencyLoop(Vec<Id>),
    Unknown,
}
