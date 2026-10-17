#[derive(Default)]
pub struct CommandStack<Command: Clone> {
	commands: Vec<Command>
}

impl<Command: Clone> CommandStack<Command> {
	pub fn push_command(&mut self, command: Command) {
		self.commands.push(command);
	}
	pub fn pop_command(&mut self) -> Option<Command> {
		self.commands.pop()
	}

	pub fn clone_commands(&self) -> Vec<Command> {
		let mut clone = Vec::default();

		for command in self.commands.iter() {
			clone.push(command.clone());
		}

		return clone;
	}
}