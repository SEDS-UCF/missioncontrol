use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::interactions::Interaction;
use serenity::model::interactions::message_component::ButtonStyle;

use crate::bot::Bot;
use crate::bot::config::{GUILD_ID, SEND_INTRO};

#[async_trait]
impl EventHandler for Bot {
	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		GUILD_ID
			.set_application_commands(&ctx.http, |commands| {
				commands
					.create_application_command(|command| {
						command
							.name("mc")
							.description("Launch Mission Control")
					})
					.create_application_command(|command| {
						command
							.name("become")
							.description("Change your membership type")
							.create_option(|option| {
								option
									.name("type")
									.description("Your new membership type")
									.kind(ApplicationCommandOptionType::String)
									.required(true)
									.add_string_choice("Current Member", "member")
									.add_string_choice("Graduated Alumnus", "alumni")
									.add_string_choice("Friend of SEDS", "friend")
							})
					})
				// .create_application_command(|command| {
				// 	command
				// 		.name("join")
				// 		.description("Join a channel")
				// 		.create_option(|option| {
				// 			option
				// 				.name("channel")
				// 				.description("The channel to join")
				// 				.kind(ApplicationCommandOptionType::String)
				// 				.required(true)
				// 		})
				// })
				// .create_application_command(|command| {
				// 	command
				// 		.name("leave")
				// 		.description("Leave a channel")
				// 		.create_option(|option| {
				// 			option
				// 				.name("channel")
				// 				.description("The channel to leave")
				// 				.kind(ApplicationCommandOptionType::Channel)
				// 				.required(true)
				// 		})
				// })
			})
			.await.unwrap();

		// This block sends the message which contains the "Launch Mission Control" button.
		// We only want to do this if the message is deleted, so guard it behind a config flag.
		if SEND_INTRO {
			ChannelId(869756293894783006).to_channel(&ctx).await.unwrap().guild().unwrap().send_message(&ctx, |f| {
				f
					.embed(|f| {
						f.description("Click the button to launch Mission Control!")
					})
					.components(|g| {
						g
							.create_action_row(|ar| {
								ar.create_button(|b| {
									b
										.label("Launch!")
										.custom_id("launch-mc")
										.style(ButtonStyle::Success)
								})
							})
					})
			}).await.unwrap();
		}
	}

	/// This is the Serenity event for all interactions. From here, we dispatch out to handle
	/// commands and components separately.
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			self.handle_command(ctx, command).await;
		} else if let Interaction::MessageComponent(component) = interaction {
			self.handle_component(ctx, component).await;
		}
	}
}