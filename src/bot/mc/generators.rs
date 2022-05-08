use serenity::builder::CreateInteractionResponseData;
use serenity::model::interactions::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::interactions::message_component::ButtonStyle;
use crate::bot::config::MAX_LIST_SIZE;

use crate::bot::mc::{MC, Modifications, State, StateProgress};

impl MC {
	pub fn generate_main_menu<'a, 'b>(&self, d: &'a mut CreateInteractionResponseData<'b>) -> &'a mut CreateInteractionResponseData<'b> {
		d.content("This code is highly unstable, please report any bugs to <@125812945061019650>!");
		d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);

		d.components(|c| {
			c.create_action_row(|ar| {
				ar
					.create_button(|b| { b.custom_id("membership").label("Membership").style(ButtonStyle::Primary) })
					.create_button(|b| { b.custom_id("roles").label("Roles").style(ButtonStyle::Primary) })
					.create_button(|b| { b.custom_id("chans").label("Channels").style(ButtonStyle::Primary) })
					.create_button(|b| { b.custom_id("projs").label("Projects").style(ButtonStyle::Primary) })
					.create_button(|b| { b.custom_id("games").label("Games").style(ButtonStyle::Primary) })
			})
			.create_action_row(|ar| {
				ar.create_button(|b| { b.custom_id("exit-mc").label("Done").style(ButtonStyle::Secondary) })
			})
		})
	}

	pub fn generate_done<'a, 'b>(&self, d: &'a mut CreateInteractionResponseData<'b>) -> &'a mut CreateInteractionResponseData<'b> {
		d.content("Goodbye!");
		d.components(|c| {
			c
		})
	}

	pub fn generate_modification<'a, 'b>(&self, d: &'a mut CreateInteractionResponseData<'b>) -> &'a mut CreateInteractionResponseData<'b> {
		match &self.state {
			State::Modification(state) => {
				match state {
					StateProgress::Initial => {
						d.components(|c| {
							c.create_action_row(|ar| {
								match self.modification.as_ref().unwrap() {
									Modifications::Membership => unreachable!(),
									Modifications::Roles => {
										ar
											.create_button(|b| { b.custom_id("add").label("Add Roles").style(ButtonStyle::Success) })
											.create_button(|b| { b.custom_id("remove").label("Remove Roles").style(ButtonStyle::Danger) })
											.create_button(|b| { b.custom_id("done").label("Done").style(ButtonStyle::Secondary) })
									}
									Modifications::Channels => {
										ar
											.create_button(|b| { b.custom_id("add").label("Join Channels").style(ButtonStyle::Success) })
											.create_button(|b| { b.custom_id("remove").label("Leave Channels").style(ButtonStyle::Danger) })
											.create_button(|b| { b.custom_id("done").label("Done").style(ButtonStyle::Secondary) })
									}
									Modifications::Projects => {
										ar
											.create_button(|b| { b.custom_id("add").label("Join Projects").style(ButtonStyle::Success) })
											.create_button(|b| { b.custom_id("remove").label("Leave Projects").style(ButtonStyle::Danger) })
											.create_button(|b| { b.custom_id("done").label("Done").style(ButtonStyle::Secondary) })
									}
									Modifications::Games => {
										ar
											.create_button(|b| { b.custom_id("add").label("Add Games").style(ButtonStyle::Success) })
											.create_button(|b| { b.custom_id("remove").label("Remove Games").style(ButtonStyle::Danger) })
											.create_button(|b| { b.custom_id("done").label("Done").style(ButtonStyle::Secondary) })
									}
								}
							})
						})
					}
					StateProgress::Add => {
						if let Some(modif) = self.modification {
							let placeholder = match modif {
								Modifications::Roles => "Select a role to add...",
								Modifications::Channels => "Select a channel to join...",
								Modifications::Projects => "Select a project to join...",
								Modifications::Games => "Select a game to add...",
								Modifications::Membership => unreachable!(),
							};

							sel_menu(d, placeholder, self.list.as_ref(), self.page)
						} else {
							unreachable!()
						}
					}
					StateProgress::Remove => {
						if let Some(modif) = self.modification {
							let placeholder = match modif {
								Modifications::Roles => "Select a role to remove...",
								Modifications::Channels => "Select a channel to leave...",
								Modifications::Projects => "Select a project to leave...",
								Modifications::Games => "Select a game to remove...",
								Modifications::Membership => unreachable!(),
							};

							sel_menu(d, placeholder, self.list.as_ref(), self.page)
						} else {
							unreachable!()
						}
					}
					StateProgress::Change => {
						if let Some(modif) = self.modification {
							if modif != Modifications::Membership {
								unreachable!()
							}

							sel_menu(d, "Select a new membership type...", self.list.as_ref(), self.page)
						} else {
							unreachable!()
						}
					}
				}
			}
			_ => unreachable!()
		}
	}
}

pub struct MenuOption {
	pub label: String,
	pub val: String,
}

fn sel_menu<'a, 'b>(d: &'a mut CreateInteractionResponseData<'b>, placehold: &str, list: &Vec<MenuOption>, _page: u8) -> &'a mut CreateInteractionResponseData<'b> {
	if list.len() > MAX_LIST_SIZE {
		error!("Warning! Trying to display a list near the maximum of 25! Time to implement paging...")
	}

	d.components(|c| {
		c.create_action_row(|ar| {
			ar.create_select_menu(|sm| {
				sm.custom_id("sel-val");
				if list.is_empty() {
					sm
						.disabled(true)
						.placeholder("There are no available options!")
						.options(|smo| smo.create_option(|o| o.label("none").value("none")))
				} else {
					sm.placeholder(placehold);
					sm.options(|smo| {
						for li in list {
							smo.create_option(|o| o.label(li.label.as_str()).value(li.val.as_str()));
						}
						smo
					})
				}
			})
		})
			.create_action_row(|ar| {
				ar
					// .create_button(|b| {
					// 	b.custom_id("prev-page").label("Previous Page").style(ButtonStyle::Primary);
					//
					// 	if page == 0 {
					// 		b.disabled(true);
					// 	}
					//
					// 	b
					// })
					// .create_button(|b| {
					// 	b.custom_id("next-page").label("Next Page").style(ButtonStyle::Primary);
					// 	b
					// })
					.create_button(|b| { b.custom_id("done").label("Done").style(ButtonStyle::Secondary) })
			})
	})
}