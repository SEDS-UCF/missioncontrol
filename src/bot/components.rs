use serenity::client::Context;
use serenity::model::interactions::message_component::MessageComponentInteraction;

use crate::bot::Bot;
use crate::bot::mc::MC;

impl Bot {
	pub async fn handle_component(&self, ctx: Context, component: MessageComponentInteraction) {
		trace!("Handling component {} from {}", component.data.custom_id, component.user.tag());
		match component.data.custom_id.as_str() {
			"launch-mc" => MC::from_component(ctx, component).await,
			_ => {}
		}
	}
}