use gtk::prelude::*;
use talk_loco_client::response::chat::LChatListRes;
use talk_loco_client::structs::channel_info::ChannelListData;
use crate::{add_channel_list, ChannelEntry, Error, gui, kiwitalk};

pub async fn chat_list_update_handler(chat_list: LChatListRes) -> Result<(), Error> {
	for channel_list_data in chat_list.chat_datas {
		let kiwitalk = kiwitalk().await;
		let members = kiwitalk.members(channel_list_data.id).await?;
		let channel_info = kiwitalk.channel_info(channel_list_data.id).await?;
		gtk::glib::idle_add_once(move || {
			for x in gui::Main::get().chats_list_box.children() {
				gui::Main::get().chats_list_box.remove(&x);
			}
			add_channel_list(&channel_list_data, members, channel_info);
		});
	}
	Ok(())
}