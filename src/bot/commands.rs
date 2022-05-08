use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

use crate::bot::Bot;
use crate::bot::config::{ALLOWED_MEMBERSHIPS, MEMBERSHIP_ALUMNI, MEMBERSHIP_FRIEND, MEMBERSHIP_MEMBER};
use crate::bot::mc::MC;
use crate::bot::mc::utils::user_change_role;

impl Bot {
	pub async fn handle_command(&self, ctx: Context, command: ApplicationCommandInteraction) {
		trace!("Handling command {} from {}", command.data.name, command.user.tag());
		match command.data.name.as_str() {
			"mc" => MC::from_command(ctx, command).await,
			"become" => Bot::handle_become(ctx, command).await,
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
}