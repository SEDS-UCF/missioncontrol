use serenity::model::id::RoleId;
use serenity::model::prelude::ChannelId;

use crate::bot::config::{ALLOWED_MEMBERSHIPS, ALLOWED_PROJECTS, ALLOWED_ROLES, CAT_CHANNELS, CAT_GAMES, GUILD_ID};
use crate::bot::mc::{MC, Modifications, State, StateProgress};
use crate::bot::mc::generators::MenuOption;
use crate::bot::mc::utils::{filter_chans, user_add_role, user_change_role, user_join_chan, user_leave_chan, user_remove_role};

impl MC {
	pub async fn process(&mut self) {
		if let State::Modification(progress) = self.state {
			// If we were given a value to process, do so now.
			if self.value.is_some() {
				self.process_val(progress).await;
				self.value = None;

				// After we do a Change (instead of an Add or Remove), we're done; go back to main.
				if progress == StateProgress::Change {
					self.state = State::MainMenu;
				}
			}

			// We always call process_list, since process_value could have changed the existing list.
			self.process_list(progress).await;
		}
	}

	async fn process_val(&mut self, progress: StateProgress) {
		match self.modification.as_ref().unwrap() {
            Modifications::Membership => {
                match progress {
                    StateProgress::Initial => unreachable!(),
	                StateProgress::Add => unreachable!(),
	                StateProgress::Remove => unreachable!(),
	                // The ONLY valid state for a Membership modification is Change.
                    StateProgress::Change => {
	                    let role: RoleId = self.value.as_ref().unwrap().parse().unwrap();
                        user_change_role(&self.ctx, &self.user, role, ALLOWED_MEMBERSHIPS).await;
                    }
                }
            }
			Modifications::Roles => {
				match progress {
					// If we're in process_val, we're already adding or removing, we can't be initial.
					StateProgress::Initial => unreachable!(),
					StateProgress::Change => unreachable!(), // Only relevant to Membership.
					StateProgress::Add => {
						let role: RoleId = self.value.as_ref().unwrap().parse().unwrap();
						user_add_role(&self.ctx, &self.user, role).await;
					}
					StateProgress::Remove => {
						let role: RoleId = self.value.as_ref().unwrap().parse().unwrap();
						user_remove_role(&self.ctx, &self.user, role).await;
					}
				}
			}
			Modifications::Channels => {
				match progress {
					// If we're in process_val, we're already adding or removing, we can't be initial.
					StateProgress::Initial => unreachable!(),
					StateProgress::Change => unreachable!(), // Only relevant to Membership.
					StateProgress::Add => {
						let chan: ChannelId = self.value.as_ref().unwrap().parse().unwrap();
						user_join_chan(&self.ctx, &self.user, chan).await;
					}
					StateProgress::Remove => {
						let chan: ChannelId = self.value.as_ref().unwrap().parse().unwrap();
						user_leave_chan(&self.ctx, &self.user, chan).await;
					}
				}
			}
			Modifications::Projects => {
				match progress {
					// If we're in process_val, we're already adding or removing, we can't be initial.
					StateProgress::Initial => unreachable!(),
					StateProgress::Change => unreachable!(), // Only relevant to Membership.
					StateProgress::Add => {
						let role: RoleId = self.value.as_ref().unwrap().parse().unwrap();
						user_add_role(&self.ctx, &self.user, role).await;
					}
					StateProgress::Remove => {
						let role: RoleId = self.value.as_ref().unwrap().parse().unwrap();
						user_remove_role(&self.ctx, &self.user, role).await;
					}
				}
			}
			Modifications::Games => {
				match progress {
					// If we're in process_val, we're already adding or removing, we can't be initial.
					StateProgress::Initial => unreachable!(),
					StateProgress::Change => unreachable!(), // Only relevant to Membership.
					StateProgress::Add => {
						let chan: ChannelId = self.value.as_ref().unwrap().parse().unwrap();
						user_join_chan(&self.ctx, &self.user, chan).await;
					}
					StateProgress::Remove => {
						let chan: ChannelId = self.value.as_ref().unwrap().parse().unwrap();
						user_leave_chan(&self.ctx, &self.user, chan).await;
					}
				}
			}
		}
	}

	async fn process_list(&mut self, progress: StateProgress) {
		match self.modification.unwrap() {
            Modifications::Membership => {
                if progress == StateProgress::Initial {
                    return;
                }

                let member = GUILD_ID.member(&self.ctx, self.user.id).await;
                if member.is_err() {
                    error!("Error retrieving member from UserId {}", self.user.id);
                    return;
                }
                let member = member.unwrap();

                let avail_roles: Vec<_> = ALLOWED_MEMBERSHIPS.iter().filter(|x| {
                    if progress == StateProgress::Change {
                        !member.roles.contains(x)
                    } else {
                        unreachable!()
                    }
                }).collect();

                self.list = avail_roles.iter().map(|x| {
                    let role = x.to_role_cached(&self.ctx);
                    let role = role.unwrap();
                    MenuOption {
                        label: role.name,
                        val: role.id.to_string(),
                    }
                }).collect();
            }
			Modifications::Roles => {
				if progress == StateProgress::Initial {
					return;
				}

				let member = GUILD_ID.member(&self.ctx, self.user.id).await;
				if member.is_err() {
					error!("Error retrieving member from UserId {}", self.user.id);
					return;
				}
				let member = member.unwrap();

                let avail_roles: Vec<_> = ALLOWED_ROLES.iter().filter(|x| {
                    if progress == StateProgress::Add {
                        !member.roles.contains(x)
                    } else if progress == StateProgress::Remove {
                        member.roles.contains(x)
                    } else {
                        unreachable!()
                    }
                }).collect();

                self.list = avail_roles.iter().map(|x| {
                    let role = x.to_role_cached(&self.ctx).unwrap();
                    MenuOption {
                        label: role.name,
                        val: role.id.to_string(),
                    }
                }).collect();
			}
			Modifications::Projects => {
				if progress == StateProgress::Initial {
					return;
				}

				let member = GUILD_ID.member(&self.ctx, self.user.id).await;

				if member.is_err() {
					error!("Error retrieving member from UserId {}", self.user.id);
					return;
				}

				let member = member.unwrap();

				let avail_roles: Vec<_> = ALLOWED_PROJECTS.iter().filter(|x| {
					if progress == StateProgress::Add {
						!member.roles.contains(x)
					} else if progress == StateProgress::Remove {
						member.roles.contains(x)
					} else {
						unreachable!()
					}
				}).collect();

				self.list = avail_roles.iter().map(|x| {
					let role = x.to_role_cached(&self.ctx).unwrap();
					MenuOption {
						label: role.name,
						val: role.id.to_string(),
					}
				}).collect();
			}
			Modifications::Channels => {
				if progress == StateProgress::Initial {
					return;
				}

				let chans = GUILD_ID.channels(&self.ctx).await.unwrap();
				let chans = filter_chans(&self.ctx, &chans, CAT_CHANNELS, self.user.id, progress);

				self.list = chans.iter().map(|x| MenuOption {
					label: x.name.clone(),
					val: x.id.to_string(),
				}).collect();
			}
			Modifications::Games => {
				if progress == StateProgress::Initial {
					return;
				}

				let chans = GUILD_ID.channels(&self.ctx).await.unwrap();
				let chans = filter_chans(&self.ctx, &chans, CAT_GAMES, self.user.id, progress);

				self.list = chans.iter().map(|x| MenuOption {
					label: x.name.clone(),
					val: x.id.to_string(),
				}).collect();
			}
		}
	}
}