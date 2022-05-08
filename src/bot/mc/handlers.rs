use std::sync::Arc;

use serenity::model::interactions::message_component::MessageComponentInteraction;

use crate::bot::mc::{MC, Modifications, State, StateProgress};

impl MC {
	pub fn handle_main_menu(&mut self, a: Arc<MessageComponentInteraction>) {
		self.state = State::Modification(StateProgress::Initial);

		match a.data.custom_id.as_str() {
			"membership" => {
				self.modification = Some(Modifications::Membership);
				self.state = State::Modification(StateProgress::Change);
			}
			"roles" => self.modification = Some(Modifications::Roles),
			"chans" => self.modification = Some(Modifications::Channels),
			"projs" => self.modification = Some(Modifications::Projects),
			"games" => self.modification = Some(Modifications::Games),
			"exit-mc" => {
				self.state = State::Done;
				self.running = false;
			}
			_ => unreachable!()
		}
	}

	pub fn handle_done(&mut self, _a: Arc<MessageComponentInteraction>) {
		unreachable!()
	}

	pub fn handle_modification(&mut self, a: Arc<MessageComponentInteraction>) {
		match &self.state {
			State::Modification(state) => {
				match state {
					StateProgress::Initial => {
						match a.data.custom_id.as_str() {
							"add" => self.state = State::Modification(StateProgress::Add),
							"remove" => self.state = State::Modification(StateProgress::Remove),
							"done" => {
								self.state = State::MainMenu;
								self.modification = None;
							}
							_ => unreachable!()
						}
					}
					StateProgress::Add => {
						match a.data.custom_id.as_str() {
							"next-page" => self.page += 1,
							"prev-page" => self.page = self.page.saturating_sub(1),
							"done" => {
								self.state = State::MainMenu;
								self.modification = None;
							}
							"sel-val" => {
								self.value = a.data.values.first().cloned();
							}
							_ => unreachable!()
						}
					}
					StateProgress::Remove => {
						match a.data.custom_id.as_str() {
							"next-page" => self.page += 1,
							"prev-page" => self.page = self.page.saturating_sub(1),
							"done" => {
								self.state = State::MainMenu;
								self.modification = None;
							}
							"sel-val" => {
								self.value = a.data.values.first().cloned();
							}
							_ => unreachable!()
						}
					}
					StateProgress::Change => {
						match a.data.custom_id.as_str() {
							"done" => {
								self.state = State::MainMenu;
								self.modification = None;
							}
							"sel-val" => {
								self.value = a.data.values.first().cloned();
							}
							_ => unreachable!()
						}
					}
				}
			}
			_ => unreachable!(),
		}
	}
}