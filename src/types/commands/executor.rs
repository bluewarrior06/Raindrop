use std::marker::PhantomData;

use crate::types::commands::{command_stack::CommandStack, implementator::Implementator};

/// Holds commands and executes them.
pub struct Executor<State, Command: Clone, Err> {
	commands: Vec<Command>,
	
	_a: PhantomData<State>,
	_c: PhantomData<Err>
}

impl<State, Command: Clone, Err> Executor<State, Command, Err> {
	pub fn execute(&mut self, state: &mut State, implementator: &mut impl Implementator<State, Command, Err>) -> Option<Err> {
		implementator.clear();
		
		for command in self.commands.iter() {
			if let Some(err) = implementator.execute(state, command) {
				return Some(err);
			}
		}
		self.commands.clear();

		return None;
	}

	pub fn push_command(&mut self, command: Command) {
		self.commands.push(command);
	}
	pub fn pop_command(&mut self) -> Option<Command> {
		self.commands.pop()
	}

	fn push_command_stack(&mut self, command_stack: &mut CommandStack<Command>) {
		let cloned_commands = command_stack.clone_commands();
		
		for cloned_command in cloned_commands {
			self.push_command(cloned_command);
		}
	}
}