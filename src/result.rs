pub type Result<Id, V> = std::result::Result<Vec<V>, Error<Id, V>>;

#[derive(PartialEq, Debug)]
pub enum Error<Id, V> {
    DependencyLoop(Vec<(Id, V)>),
    Unknown,
}
