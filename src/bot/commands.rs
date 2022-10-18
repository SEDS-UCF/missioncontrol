use serenity::client::Context;
use serenity::model::channel::ChannelType;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};
use serenity::model::prelude::ChannelId;

use crate::bot::Bot;
use crate::bot::config::{ALLOWED_MEMBERSHIPS, CAT_CHANNELS, GUILD_ID, MEMBERSHIP_ALUMNI, MEMBERSHIP_FRIEND, MEMBERSHIP_MEMBER};
use crate::bot::mc::MC;
use crate::bot::mc::utils::{user_change_role, user_join_chan, user_leave_chan};

impl Bot {
	pub async fn handle_command(&self, ctx: Context, command: ApplicationCommandInteraction) {
		trace!("Handling command {} from {}", command.data.name, command.user.tag());
		match command.data.name.as_str() {
			"mc" => MC::from_command(ctx, command).await,
			"become" => Bot::handle_become(ctx, command).await,
			"join" => Bot::handle_join(ctx, command).await,
			"leave" => Bot::handle_leave(ctx, command).await,
			_ => error!("Received an unimplemented command {}!", command.data.name.as_str()),
		};
	}

	async fn handle_become(ctx: Context, command: ApplicationCommandInteraction) {
		let opt = command.data.options.first();
		if opt.is_none() {
			error!("Somehow called /become with no option!");
			return;
		}
		let opt = opt.unwrap();

		let choice = opt.value.as_ref().unwrap().as_str().unwrap();

		debug!("{} called /become with: {}", command.user.tag(), choice);

		match choice {
			"member" => {
				user_change_role(&ctx, &command.user, MEMBERSHIP_MEMBER, ALLOWED_MEMBERSHIPS).await;
			}
			"alumni" => {
				user_change_role(&ctx, &command.user, MEMBERSHIP_ALUMNI, ALLOWED_MEMBERSHIPS).await;
			}
			"friend" => {
				user_change_role(&ctx, &command.user, MEMBERSHIP_FRIEND, ALLOWED_MEMBERSHIPS).await;
			}
			_ => {
				error!("Somehow sent an invalid choice for /become: {}", choice);
			}
		}
	}

	async fn handle_join(ctx: Context, command: ApplicationCommandInteraction) {
		let opt = command.data.options.first();
		if opt.is_none() {
			error!("Somehow called /join with no option!");
			return;
		}
		let opt = opt.unwrap();

		let choice = opt.value.as_ref().unwrap().as_str().unwrap();

		let channel = choice.strip_prefix("#").unwrap_or(choice);

		debug!("{} called /join with: {}", command.user.tag(), choice);

		let chans = GUILD_ID.channels(&ctx).await.unwrap();
		let chans: Vec<_>= chans
			.values()
			.filter(|x| {
				if let Some(pid) = x.parent_id {
					if pid == CAT_CHANNELS && x.kind == ChannelType::Text {
						return true;
					}
				}

				false
			})
			.collect();

		let selected_chan = chans.iter().find(|c| c.name == channel);

		match selected_chan {
			None => {
				command.create_interaction_response(&ctx.http, |r| {
					r.kind(InteractionResponseType::ChannelMessageWithSource);
					r.interaction_response_data(|d| {
						d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
						d.content(format!("Error: \"{}\" is not a joinable channel!", channel))
					})
				}).await.unwrap();
			}
			Some(chan) => {
				user_join_chan(&ctx, &command.user, chan.id).await;

				command.create_interaction_response(&ctx.http, |r| {
					r.kind(InteractionResponseType::ChannelMessageWithSource);
					r.interaction_response_data(|d| {
						d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
						d.content(format!("You've successfully joined <#{}>!", chan.id.0))
					})
				}).await.unwrap();
			}
		}
	}

	async fn handle_leave(ctx: Context, command: ApplicationCommandInteraction) {
		let opt = command.data.options.first();
		if opt.is_none() {
			error!("Somehow called /leave with no option!");
			return;
		}
		let opt = opt.unwrap();

		let choice = opt.value.as_ref().unwrap().as_str().unwrap();
		let channel: ChannelId = choice.parse().unwrap();

		debug!("{} called /leave with: {}", command.user.tag(), choice);

		let chans = GUILD_ID.channels(&ctx).await.unwrap();
		let chans: Vec<_>= chans
			.values()
			.filter(|x| {
				if let Some(pid) = x.parent_id {
					if pid == CAT_CHANNELS && x.kind == ChannelType::Text {
						return true;
					}
				}

				false
			})
			.collect();

		let selected_chan = chans.iter().find(|c| c.id == channel);

		match selected_chan {
			None => {
				command.create_interaction_response(&ctx.http, |r| {
					r.kind(InteractionResponseType::ChannelMessageWithSource);
					r.interaction_response_data(|d| {
						d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
						d.content(format!("Error: <#{}> is not a leavable channel!", channel))
					})
				}).await.unwrap();
			}
			Some(chan) => {
				user_leave_chan(&ctx, &command.user, chan.id).await;

				command.create_interaction_response(&ctx.http, |r| {
					r.kind(InteractionResponseType::ChannelMessageWithSource);
					r.interaction_response_data(|d| {
						d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
						d.content(format!("You've successfully left #{}!", chan.name))
					})
				}).await.unwrap();
			}
		}
	}
}