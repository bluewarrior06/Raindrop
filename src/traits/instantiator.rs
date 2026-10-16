/// To create an instance of an object with type Type.
pub trait Instantiator
{
	/// The type to be instantiated.
	type Type;
	/// The type of when an error occurs during instantiation.
	type Err;

	fn instantiate() -> Result<Self::Type, Self::Err>;
}