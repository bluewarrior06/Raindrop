pub trait Implementator<State, Command, Err> {
	fn clear(&mut self);
	fn execute(&mut self, state: &mut State, command: &Command) -> Option<Err>;
}