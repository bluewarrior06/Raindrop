/// Implementee is capable of mapping Ele to a position at Map.
pub trait MapperCopy<Map, Ele, Err>
where Map: Clone + Copy {
	/// If the position is in bounds.
	/// If the position is out of bounds, this will return why.
	fn is_in_bounds(&self, pos: Map) -> Option<Err>;
	/// If the size is valid.
	/// If the size is out of bounds, this will return why.
	fn is_size_valid(size: Map) -> Option<Err>;

	/// Attempts to get an element at position.
	fn get(&self, pos: Map) -> Result<Ele, Err>;
	/// Attempts to set an element at position.
	fn set(&mut self, pos: Map, element: Ele) -> Option<Err>;
	
	/// Gets the index.
	/// Note: This typically wont do any bounds checks itself, and thus will return an invalid index if the position is out of bounds.
	fn index_of(&self, pos: Map) -> usize;
	/// Attempts to resize the mapper.
	fn resize(&mut self, size: Map) -> Option<Err>;
	fn get_size(&self) -> Map;
}