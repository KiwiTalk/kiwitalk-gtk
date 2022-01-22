use gtk::ListBoxRow;
use gtk::prelude::*;
use talk_loco_client::response::chat::Msg;
use talk_loco_client::structs::channel_info::ChannelListData;
use crate::{add_channel_list, ChannelEntry, Error, gui, kiwitalk};

pub async fn message_handler(msg: Msg) -> Result<(), Error> {
	let (sender, receiver) = tokio::sync::oneshot::channel();
	enum Res {
		Found,
		NotFound(Msg)
	}
	gtk::glib::idle_add_once(move || {
		sender.send(if let Some(entry) = gui::Main::get().chats_list_box.children().into_iter()
			.map(|x| x.downcast::<ListBoxRow>().unwrap().child().unwrap().downcast::<ChannelEntry>().unwrap())
			.find(|x| x.id() == msg.chat_id) {
			entry.on_msg(msg);
			Res::Found
		} else {
			Res::NotFound(msg)
		}).ok();
	});
	if let Res::NotFound(msg) = receiver.await.unwrap() {
		let kiwitalk = kiwitalk().await;
		let channel_info = kiwitalk.channel_info(msg.chat_id).await.unwrap();
		let members = kiwitalk.members(msg.chat_id).await.unwrap();
		let channel_list_data = ChannelListData {
			id: msg.chat_id,
			channel_type: channel_info.channel_type.clone(),
			last_log_id: channel_info.last_log_id,
			chatlog: None,
			member_count: channel_info.active_member_count,
			unread_count: 0,
			push_alert: channel_info.push_alert,
			link: channel_info.link.clone(),
			icon_user_ids: None,
			icon_user_nicknames: None,
			mmr: 0,
			s: 0,
			open_token: None,
			jn: None
		};
		gtk::glib::idle_add_once(move || {
			add_channel_list(&channel_list_data, members, channel_info).on_msg(msg);
		});
	}
	Ok(())
}