/// To create an instance of an object with type Type.
pub trait Instantiator<T, Err> {

    fn instantiate(&self) -> Result<T, Err>;
}
