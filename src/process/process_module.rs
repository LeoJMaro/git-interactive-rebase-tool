use crate::git_interactive::GitInteractive;
use crate::input::{Input, InputHandler};
use crate::process::{HandleInputResult, ProcessResult, State};
use crate::view::View;

pub trait ProcessModule {
	fn activate(&mut self, _state: State, _git_interactive: &GitInteractive) {}

	fn deactivate(&mut self) {}

	fn process(&mut self, _git_interactive: &mut GitInteractive, _view: &View) -> ProcessResult {
		ProcessResult::new()
	}

	fn handle_input(
		&mut self,
		_input_handler: &InputHandler,
		_git_interactive: &mut GitInteractive,
		_view: &View,
	) -> HandleInputResult
	{
		HandleInputResult::new(Input::Other)
	}

	fn render(&self, _view: &View, _git_interactive: &GitInteractive);
}
