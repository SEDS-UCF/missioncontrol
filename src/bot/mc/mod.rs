use std::sync::Arc;
use std::time::Duration;

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseData};
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::user::User;

use crate::bot::mc::generators::MenuOption;

mod handlers;
mod processor;
mod generators;
pub mod utils;

#[derive(Copy, Clone, PartialEq)]
pub enum StateProgress {
	Initial,
	Add,
	Remove,
	Change,
}

#[derive(Copy, Clone)]
pub enum State {
	MainMenu,
	Modification(StateProgress),
	Done,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Modifications {
	Membership,
	Roles,
	Channels,
	Projects,
	Games,
}

impl State {
	/// Return a handle function pointer for a given state.
	fn handler(&self) -> fn(&mut MC, Arc<MessageComponentInteraction>) {
		match self {
			State::MainMenu => MC::handle_main_menu,
			State::Modification(_) => MC::handle_modification,
			State::Done => MC::handle_done,
		}
	}

	/// Return a generator function pointer for a given state.
	fn generator(&self) -> for<'a, 'b, 'r> fn(&'r MC, &'a mut CreateInteractionResponseData<'b>) -> &'a mut CreateInteractionResponseData<'b> {
		match self {
			State::MainMenu => MC::generate_main_menu,
			State::Modification(_) => MC::generate_modification,
			State::Done => MC::generate_done,
		}
	}
}

// This whole struct is a mess. I'm still figuring out the best way to flow data in async Rust.
pub struct MC {
	/// The Serenity context.
	ctx: Context,

	/// Our bot message sent in response to the interaction.
	mess: Option<Message>,

	/// A ulid to represent this specific MC instance.
	ulid: rusty_ulid::Ulid,

	/// The user who started the interaction.
	user: User,

	/// Which state we're currently in: MainMenu, Modification, or Done.
	state: State,

	/// If we're doing a modification, this states which kind.
	modification: Option<Modifications>,

	// Now we get to the fun part...

	/// If we processed an interaction and it provided a value to process, this contains it.
	value: Option<String>,

	/// Eventually: stores the current pagination value. Currently does nothing.
	page: u8,

	/// This contains the list of values to processor generated, used by the generator to build the message.
	list: Vec<MenuOption>,

	/// Are we still running? Or has the user clicked Done on the main menu?
	running: bool,
}

impl MC {
	/// Start an MC instance from the /mc command.
	pub async fn from_command(ctx: Context, command: ApplicationCommandInteraction) {
		let mut mc = Self {
			ctx,
			mess: None,
			ulid: rusty_ulid::Ulid::generate(),
			user: command.user.clone(),
			state: State::MainMenu,
			modification: None,
			value: None,
			page: 0,
			list: vec![],
			running: true,
		};

		command.create_interaction_response(&mc.ctx, |r| mc.initial_resp(r)).await.unwrap();
		mc.mess = command.get_interaction_response(&mc.ctx).await.ok();

		mc.run().await;
	}

	/// Start an MC instance from the Launch! button being clicked.
	pub async fn from_component(ctx: Context, component: MessageComponentInteraction) {
		let mut mc = Self {
			ctx,
			mess: None,
			ulid: rusty_ulid::Ulid::generate(),
			user: component.user.clone(),
			state: State::MainMenu,
			modification: None,
			value: None,
			page: 0,
			list: vec![],
			running: true,
		};

		component.create_interaction_response(&mc.ctx, |r| mc.initial_resp(r)).await.unwrap();
		mc.mess = component.get_interaction_response(&mc.ctx).await.ok();

		mc.run().await;
	}

	async fn run(&mut self) {
		debug!("MC#{}: Created by {}", self.ulid, self.user.tag());

		while self.running {
			match &self.mess {
				None => {
					error!("We got to run() without sending an initial response!");
					return;
				},

				// This is the core loop:
				Some(mess) => {
					// Await an interaction to our response message.
					let mci = match mess
						.await_component_interaction(&self.ctx)
						.timeout(Duration::from_secs(3600))
						.await {
						Some(ci) => ci,
						None => {
							debug!("MC#{}: Interaction timeout!", self.ulid);
							return;
						}
					};

					trace!("MC#{}: Received component ID \"{}\", processing...", self.ulid, mci.data.custom_id);

					// Send the interaction of to the handler for the current state.
					(self.state.handler())(self, mci.clone());

					// Call the processor.
					self.process().await;

					// Call the generator for the current state to build a response.
					mci.create_interaction_response(&self.ctx, |f| {
						f.kind(InteractionResponseType::UpdateMessage);
						f.interaction_response_data(|g| {
							(self.state.generator())(self, g)
						})
					}).await.unwrap();
				}
			}
		}

		debug!("MC#{}: Exited gracefully!", self.ulid);
	}

	fn initial_resp<'a, 'b>(&self, r: &'a mut CreateInteractionResponse<'b>) -> &'a mut CreateInteractionResponse<'b> {
		r.kind(InteractionResponseType::ChannelMessageWithSource);
		r.interaction_response_data(|g| {
			(self.state.generator())(self, g)
		})
	}
}