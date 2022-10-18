use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::channel::{Channel, ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::Permissions;
use serenity::model::user::User;

use crate::bot::config::{EXCLUDED_CHANNELS, GUILD_ID};
use crate::bot::mc::StateProgress;

pub fn user_in_chan(ctx: &Context, user: UserId, channel: &GuildChannel) -> bool {
	channel
		.permissions_for_user(ctx, user)
		.map(|p| p.contains(Permissions::VIEW_CHANNEL))
		.unwrap_or(false)
}

pub fn filter_chans<'a>(ctx: &Context, chans: &'a HashMap<ChannelId, GuildChannel>, cat: ChannelId, user: UserId, progress: StateProgress, allow_excluded: bool) -> Vec<&'a GuildChannel> {
	let mut ret: Vec<_> = chans
		.values()
		.filter(|x| {
			let mut filt = false;

			if let Some(pid) = x.parent_id {
				if pid == cat && x.kind == ChannelType::Text && (allow_excluded || !EXCLUDED_CHANNELS.contains(&x.id)) {
					if progress == StateProgress::Add {
						filt = !user_in_chan(ctx, user, x);
					} else if progress == StateProgress::Remove {
						filt = user_in_chan(ctx, user, x);
					}
				}
			}

			filt
		})
		.collect();

	ret.sort_by(|a, b| a.position.cmp(&b.position));

	ret
}

pub async fn user_change_role(ctx: &Context, user: &User, role: RoleId, roles: &[RoleId]) {
	let member = GUILD_ID.member(ctx, user.id).await;
	if member.is_err() {
		error!("Error retrieving member from UserId {}", user.id);
		return;
	}
	let mut member = member.unwrap();

	match member.remove_roles(ctx, roles).await {
		Ok(_) => {}
		Err(_) => {
			error!("Error stripping user {} of all membership roles!", user.tag());
		}
	};

	match member.add_role(ctx, role).await {
		Ok(_) => {
			info!("Giving user {} role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
		Err(_) => {
			error!("Error giving user {} role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
	}
}

pub async fn user_add_role(ctx: &Context, user: &User, role: RoleId) {
	let member = GUILD_ID.member(ctx, user.id).await;
	if member.is_err() {
		error!("Error retrieving member from UserId {}", user.id);
		return;
	}
	let mut member = member.unwrap();

	match member.add_role(ctx, role).await {
		Ok(_) => {
			info!("Giving user {} role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
		Err(_) => {
			error!("Error giving user {} role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
	}
}

pub async fn user_remove_role(ctx: &Context, user: &User, role: RoleId) {
	let member = GUILD_ID.member(ctx, user.id).await;
	if member.is_err() {
		error!("Error retrieving member from UserId {}", user.id);
		return;
	}
	let mut member = member.unwrap();

	match member.remove_role(ctx, role).await {
		Ok(_) => {
			info!("Stripping user {} of role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
		Err(_) => {
			error!("Error stripping user {} of role {}", user.tag(), role.to_role_cached(ctx).unwrap().name);
		}
	}
}

pub async fn user_join_chan(ctx: &Context, user: &User, cid: ChannelId) {
	let chan = cid.to_channel(ctx).await;
	if chan.is_err() {
		error!("Error retrieving channel from ChannelId {}", cid);
		return;
	}
	let chan = chan.unwrap();

	match chan {
		Channel::Private(_) => { error!("Somehow got a private DM ChannelId?! {}", cid); }
		Channel::Category(_) => { error!("Somehow got a category ChannelId?! {}", cid); }
		Channel::Guild(gchan) => {
			let overwrite = PermissionOverwrite {
				allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
				deny: Permissions::empty(),
				kind: PermissionOverwriteType::Member(user.id),
			};

			match gchan.create_permission(ctx, &overwrite).await {
				Ok(_) => {
					info!("Added user {} to channel {}", user.tag(), gchan.name())
				}
				Err(_) => {
					error!("Error adding user {} to channel {}", user.tag(), gchan.name())
				}
			};
		}
		_ => {}
	}
}

pub async fn user_leave_chan(ctx: &Context, user: &User, cid: ChannelId) {
	let chan = cid.to_channel(ctx).await;
	if chan.is_err() {
		error!("Error retrieving channel from ChannelId {}", cid);
		return;
	}
	let chan = chan.unwrap();

	match chan {
		Channel::Private(_) => { error!("Somehow got a private DM ChannelId?! {}", cid); }
		Channel::Category(_) => { error!("Somehow got a category ChannelId?! {}", cid); }
		Channel::Guild(gchan) => {
			match gchan.delete_permission(ctx, PermissionOverwriteType::Member(user.id)).await {
				Ok(_) => {
					info!("Removing user {} from channel {}", user.tag(), gchan.name())
				}
				Err(_) => {
					error!("Error removing user {} from channel {}", user.tag(), gchan.name())
				}
			};
		}
		_ => {}
	}
}